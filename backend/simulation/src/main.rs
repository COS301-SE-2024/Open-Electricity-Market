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
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::content;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{data, serde, Request, Response, State};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct CORS;
pub mod grid;

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

#[get("/")]
fn index() -> String {
    "Yay".to_string()
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
fn stats(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    // let g;
    let g = grid.lock().unwrap();

    let stats = g.get_grid_stats();
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
fn start(grid: &State<Arc<Mutex<Grid>>>) -> String {
    let mut g = grid.lock().unwrap();
    if !g.started {
        g.started = true;
        let clone = grid.inner().clone();
        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut elapsed_time = 0.0;
            loop {
                let duration = start.elapsed();
                elapsed_time += duration.as_secs_f32();
                start = Instant::now();
                let mut grid = clone.lock().unwrap();
                grid.update(elapsed_time);
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                index,
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
                transformers: vec![],
            }],
            frequency: 50.0,
            started: false,
        })))
}
