#[macro_use]
extern crate rocket;

use crate::grid::circuit::Circuit;
use crate::grid::generator::Generator;
use crate::grid::load::Connection::Parallel;
use crate::grid::load::{Consumer, Load, LoadType};
use crate::grid::location::Location;
use crate::grid::{
    ConsumerInterface, GeneratorInterface, Grid, OscilloscopeDetail, Resistance, Voltage,
    VoltageWrapper,
};
use ::std::env;
use chrono::Duration;
use claims::Claims;
use core::time;
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::{insert_into, PgConnection};
use dotenvy::dotenv;
use grid::transformer::Transformer;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::content;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response, State};
use schema::open_em::grid_history::{self, grid_state};
use uuid::Uuid;

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub struct CORS;
pub mod claims;
pub mod grid;
pub mod models;
pub mod schema;

const TOKEN_EXPIRATION: Duration = Duration::minutes(15);

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

fn check_password(password: String) -> bool {
    dotenv().ok();
    let correct_password = env::var("GRID_PASS").unwrap();
    password == correct_password
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct GetTokenDetail {
    email: String,
    password: String,
}

#[post("/get_token", format = "application/json", data = "<data>")]
fn get_token(
    email_records: &State<Arc<Mutex<Vec<EmailRecord>>>>,
    data: Json<GetTokenDetail>,
) -> content::RawJson<String> {
    if !check_password(data.password.clone()) {
        let message = "Something went wrong";
        return content::RawJson(json!({"message":message}).to_string());
    }
    let mut email_records = email_records.lock().unwrap();
    let pos = email_records.iter().position(|r| r.email == data.email);
    match pos {
        Some(i) => {
            let uuid = email_records[i].id;
            let claims = Claims::from_name(uuid.to_string());
            let token = claims.into_token().unwrap();
            content::RawJson(json!({"token":token}).to_string())
        }
        None => {
            let uuid = Uuid::new_v4();
            let claims = Claims::from_name(uuid.to_string());
            let token = claims.into_token().unwrap();
            email_records.push(EmailRecord {
                id: uuid,
                email: data.email.clone(),
            });
            content::RawJson(json!({"token":token}).to_string())
        }
    }
}

#[post("/set_consumer", format = "application/json", data = "<data>")]
fn set_consumer(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<ConsumerInterface>,
    claim: Claims,
    owns: &State<Arc<Mutex<Vec<ConsumerOwner>>>>,
) -> content::RawJson<String> {
    let uuid = Uuid::parse_str(&claim.id).unwrap();
    let mut is_owned_by = false;
    {
        let owns = owns.lock().unwrap();
        for gen in owns.iter() {
            if gen.id == uuid && data.circuit == gen.circuit && data.consumer == gen.consumer {
                is_owned_by = true;
            }
        }
    }
    if !is_owned_by {
        return content::RawJson(
            json!({"status" : "err","message" : "Something went wrong"}).to_string(),
        );
    }

    let mut g = grid.lock().unwrap();
    g.set_consumer(data.into_inner());
    content::RawJson(json!({"status" : "ok","message" : "succesfully set"}).to_string())
}

#[post("/set_generator", format = "application/json", data = "<data>")]
fn set_generator(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<GeneratorInterface>,
    owns: &State<Arc<Mutex<Vec<GeneratorOwner>>>>,
    claim: Claims,
) -> content::RawJson<String> {
    let mut is_owned_by = false;
    let uuid = Uuid::parse_str(&claim.id).unwrap();
    {
        let owns = owns.lock().unwrap();

        for gen in owns.iter() {
            if gen.id == uuid && data.circuit == gen.circuit && data.generator == gen.genrator {
                is_owned_by = true;
            }
        }
    }
    if !is_owned_by {
        return content::RawJson(
            json!({"status" : "err","message" : "Something went wrong"}).to_string(),
        );
    }

    let mut g = grid.lock().unwrap();
    g.set_generator(data.into_inner());
    content::RawJson(json!({"status" : "ok","message" : "succesfully set"}).to_string())
}

#[derive(Deserialize, Clone)]
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
    owns: &State<Arc<Mutex<Vec<ConsumerOwner>>>>,
    claim: Claims,
) -> content::RawJson<String> {
    let uuid = Uuid::parse_str(&claim.id).unwrap();
    {
        let owns = owns.lock().unwrap();
        for con in owns.iter() {
            if con.id == uuid && data.latitude == con.latitude && data.longitude == con.longitude {
                let new_consumer = NewConsumer {
                    circuit: con.circuit,
                    consumer: con.consumer,
                };
                let out = serde_json::to_string(&new_consumer).unwrap();
                return content::RawJson(out);
            }
        }
    }
    let mut grid = grid.lock().unwrap();

    let (consumer, circuit) = grid.create_consumer(data.latitude, data.longitude);

    let new_consumer = NewConsumer { circuit, consumer };
    {
        let mut owns = owns.lock().unwrap();
        owns.push(ConsumerOwner {
            id: uuid,
            circuit: new_consumer.circuit,
            consumer: new_consumer.consumer,
            latitude: data.latitude,
            longitude: data.longitude,
        });
    }

    let out = serde_json::to_string(&new_consumer).unwrap();
    println!("Added consumer");

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
    owns: &State<Arc<Mutex<Vec<GeneratorOwner>>>>,
    claim: Claims,
) -> content::RawJson<String> {
    let uuid = Uuid::parse_str(&claim.id).unwrap();
    {
        let owns = owns.lock().unwrap();
        for gen in owns.iter() {
            if gen.id == uuid && data.latitude == gen.latitude && data.longitude == gen.longitude {
                let new_generator = NewGenerator {
                    circuit: gen.circuit,
                    generator: gen.genrator,
                };
                let out = serde_json::to_string(&new_generator).unwrap();
                return content::RawJson(out);
            }
        }
    }

    let mut grid = grid.lock().unwrap();

    let (circuit, generator) = grid.create_producer(data.latitude, data.longitude);

    let new_genenrator = NewGenerator { circuit, generator };
    {
        let mut owns = owns.lock().unwrap();
        owns.push(GeneratorOwner {
            id: uuid,
            circuit: new_genenrator.circuit,
            genrator: new_genenrator.generator,
            latitude: data.latitude,
            longitude: data.longitude,
        });
    }

    let out = serde_json::to_string(&new_genenrator).unwrap();

    println!("Added generator");

    content::RawJson(out)
}

#[post("/current_voltage")]
fn current_voltage(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    let grid = grid.lock().unwrap();
    let out = grid.circuits[0].loads[0]
        .get_voltage()
        .oscilloscope_detail
        .amplitude;
    content::RawJson(
        json!({"status" : "ok","message" : "Voltage returned", "data" :out}).to_string(),
    )
}

#[post("/stats")]
fn stats(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    let grid = grid.lock().unwrap();
    let stats = grid.get_grid_stats();
    let stats = serde_json::to_string(&stats).unwrap();
    content::RawJson(stats)
}

#[post("/info", format = "application/json")]
fn info(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    let grid = grid.lock().unwrap();
    let info = serde_json::to_string(grid.deref()).unwrap();
    content::RawJson(info)
}

#[post("/start", format = "application/json")]
fn start(grid: &State<Arc<Mutex<Grid>>>) -> String {
    let mut g = grid.lock().unwrap();
    if !g.started {
        g.started = true;
        let grid_clone = grid.inner().clone();
        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut elapsed_time = 0.0;
            let mut count = 0;
            loop {
                //Time calc
                let duration = start.elapsed();
                elapsed_time += duration.as_secs_f32();
                start = Instant::now();
                {
                    let mut grid = grid_clone.lock().unwrap();
                    //Update grid
                    grid.update(elapsed_time);
                    //Save to database
                    if elapsed_time > count as f32 * 50.0 {
                        use crate::grid_history::dsl::grid_history;
                        count += 1;
                        let serialized_data: serde_json::Value =
                            serde_json::from_str(&serde_json::to_string(grid.deref()).unwrap())
                                .expect("REASON");
                        let _ = insert_into(grid_history)
                            .values(grid_state.eq(serialized_data))
                            .execute(&mut establish_connection());
                        println!("Stored to database");
                    }
                }
                thread::sleep(time::Duration::from_millis(16));
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

struct EmailRecord {
    id: Uuid,
    email: String,
}

struct GeneratorOwner {
    id: Uuid,
    circuit: u32,
    genrator: u32,
    latitude: f32,
    longitude: f32,
}

struct ConsumerOwner {
    id: Uuid,
    circuit: u32,
    consumer: u32,
    latitude: f32,
    longitude: f32,
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
            latitude: -25.7563,
            longitude: 28.2373,
        },
        target: None,
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
                current_voltage,
                get_token
            ],
        )
        .manage(Arc::new(Mutex::new(Vec::<ConsumerOwner>::new())))
        .manage(Arc::new(Mutex::new(Vec::<GeneratorOwner>::new())))
        .manage(Arc::new(Mutex::new(Vec::<EmailRecord>::new())))
        .manage(Arc::new(Mutex::new(Grid {
            circuits: vec![
                Circuit {
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
                                    latitude: -25.755_686,
                                    longitude: 28.231_646,
                                },
                            }),
                            id: 0,
                        },
                        Load {
                            load_type: LoadType::new_transmission_line(80.0, -25.724_64, 28.25625),
                            id: 1,
                        },
                    ],
                    connections: vec![Parallel(0, 1)],
                    generators: vec![Generator {
                        id: 2,
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
                            // (latitude is first, measures y-axis, and for SA is negative)
                            // long: 28.048782348632816, lat: -26.120609901056977
                            latitude: -26.2044,
                            longitude: 28.0248,
                        },
                    }],
                    transformers: vec![trans_ref.clone()],
                },
                Circuit {
                    id: 1,
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
                                    latitude: -25.7331,
                                    longitude: 28.2473,
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
                            // long: 28.048782348632816, lat: -26.120609901056977
                            latitude: -25.7331,
                            longitude: 28.1473,
                        },
                    }],

                    transformers: vec![trans_ref.clone()],
                },
            ],
            frequency: 50.0,
            started: false,
        })))
}
