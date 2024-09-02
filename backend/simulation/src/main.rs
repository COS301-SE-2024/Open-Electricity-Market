#[macro_use]
extern crate rocket;

use crate::grid::circuit::Circuit;
use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Consumer, Load, LoadType};
use crate::grid::location::Location;
use crate::grid::{
    ConsumerInterface, GeneratorInterface, Grid, OscilloscopeDetail, Resistance, Voltage,
    VoltageWrapper,
};
use ::std::env;
use core::time;
use std::sync::mpsc::{self, sync_channel, Receiver, SyncSender};
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::{insert_into, PgConnection};
use dotenvy::dotenv;
use grid::transformer::Transformer;
use grid::GridStats;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::content;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{data, serde, Request, Response, State};
use schema::open_em::grid_history::{self, grid_state};
use std::any::Any;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub struct CORS;
pub mod grid;
pub mod models;
pub mod schema;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[post("/set_consumer", format = "application/json", data = "<data>")]
fn set_consumer(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<ConsumerInterface>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();
    g.set_consumer(data.into_inner());
    return content::RawJson(json!({"status" : "ok","message" : "succesfully set"}).to_string());
}

#[post("/set_generator", format = "application/json", data = "<data>")]
fn set_generator(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<GeneratorInterface>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();
    g.set_generator(data.into_inner());
    return content::RawJson(json!({"status" : "ok","message" : "succesfully set"}).to_string());
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddLocation {
    latitude: f32,
    longitude: f32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NewConsumer {
    circuit: u32,
    consumer: u32,
}

#[post("/add_consumer", format = "application/json", data = "<data>")]
fn add_consumer(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<AddLocation>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();
    let (consumer, circuit) = g.create_consumer(data.latitude, data.longitude);

    let new_consumer = NewConsumer { circuit, consumer };

    let out = serde_json::to_string(&new_consumer).unwrap();

    content::RawJson(out)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct NewGenerator {
    circuit: u32,
    generator: u32,
}

#[post("/add_generator", format = "application/json", data = "<data>")]
fn add_generator(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<AddLocation>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();

    let (circuit, generator) = g.create_producer(data.latitude, data.longitude);

    let new_genenrator = NewGenerator { circuit, generator };

    let out = serde_json::to_string(&new_genenrator).unwrap();
    println!("{generator}");

    content::RawJson(out)
}

#[post("/current_voltage")]
fn current_voltage(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    let grid = grid.lock().unwrap();
    let out = grid.circuits[0].loads[0]
        .get_voltage()
        .oscilloscope_detail
        .amplitude;
    return content::RawJson(
        json!({"status" : "ok","message" : "Voltage returned", "data" :out}).to_string(),
    );
}

#[post("/stats")]
fn stats(man: &State<Arc<Mutex<ChannelManager>>>) -> content::RawJson<String> {   
    let (tx,rx) = mpsc::sync_channel(1);
    {
        let mut manager = man.lock().unwrap();
        manager.stats.as_mut().unwrap().push(tx);
    }

    let stats = rx.recv().unwrap();
    let stats = serde_json::to_string(&stats).unwrap();
    content::RawJson(stats)
}

#[post("/info", format = "application/json")]
fn info(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    match grid.lock() {
        Ok(g) => {
            let info = serde_json::to_string(g.deref()).unwrap();
            content::RawJson(info)
        }
        Err(err) => content::RawJson(err.to_string()),
    }
}

#[post("/start", format = "application/json")]
fn start(grid: &State<Arc<Mutex<Grid>>>,man: &State<Arc<Mutex<ChannelManager>>>) -> String {
    let mut g = grid.lock().unwrap();
    if !g.started {
        g.started = true;
        let grid_clone = grid.inner().clone();
        let manager_clone = man.inner().clone();
        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut elapsed_time = 0.0;
            let mut count = 0;
            loop {
                //Time calc
                let duration = start.elapsed();
                elapsed_time += duration.as_secs_f32();
                start = Instant::now();
                let mut grid = grid_clone.lock().unwrap();
                //Update grid
                grid.update(elapsed_time);
                //Save to database
                if elapsed_time > count as f32*50.0 {
                    use crate::grid_history::dsl::grid_history;
                    count+=1;
                    let serialized_data: serde_json::Value =
                        serde_json::from_str(&serde_json::to_string(grid.deref()).unwrap())
                            .expect("REASON");
                    println!("Stored to database {}",serialized_data.clone());
                    let _ = insert_into(grid_history)
                        .values(grid_state.eq(serialized_data))
                        .execute(&mut establish_connection());
                }
                //Transfer stats
                let stats_tx;
                {
                let mut manager =manager_clone.lock().unwrap();
                stats_tx = manager.stats.take().unwrap();
                manager.stats = Some(vec![]); 
                }

                for tx in stats_tx {
                    tx.send(grid.get_grid_stats()).unwrap();
                }
                }
        });
 
        json!({
            "Message": "Started Grid"
        })
        .to_string()
    } else {
        json!({
            "Message": "Grid Already Running"
        })
        .to_string()
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct ChannelManager {
   stats : Option<Vec<SyncSender<GridStats>>>
}

#[launch]
fn rocket() -> _ {
    let transformer = Transformer {
        id: 0,
        ratio: 1.0,
        primary_circuit: 0,
        secondary_circuit: 1,
        primary_load: 0,
        secondary_voltage: VoltageWrapper {
            voltage: Voltage(0.0, 0.0, 0.0),
            oscilloscope_detail: OscilloscopeDetail {
                frequency: 0.0,
                amplitude: 0.0,
                phase: 0.0,
            },
        },
        location: Location {
            latitude: 0.0,
            longitude: 0.0,
        },
    };
    let trans_ref = Arc::new(Mutex::new(transformer));

    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                start,
                info,
                set_generator,
                stats,
                add_generator,
                add_consumer,
                set_consumer,
                current_voltage
            ],
        )
        .manage(Arc::new(Mutex::new(ChannelManager { stats: Some(vec![]) })))
        .manage(Arc::new(Mutex::new(Grid {
            circuits: vec![Circuit {
                id: 0,
                loads: vec![
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 0,
                            resistance: Resistance(10.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2044,
                                longitude: 28.0248,
                            },
                        }),
                        id: 0,
                    },
                    Load {
                        load_type: LoadType::new_transmission_line(80.0, -26.3044, 28.1),
                        id: 1,
                    },
                ],
                connections: vec![Parallel(0, 1)],
                generators: vec![Generator {
                    id: 0,
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    max_voltage: 240.0,
                    frequency: 50.0,
                    transmission_line: 0,
                    location: Location {
                        // reference point:
                        // lat: 28.048782348632816, long: -26.120609901056977
                        latitude: 28.04878,
                        longitude: -26.12061,
                    },
                }],
                transformers: vec![trans_ref.clone()],
            }],
            frequency: 50.0,
            started: false,
        })))
}
