#[macro_use]
extern crate rocket;

use crate::grid::generator::Generator;
use crate::grid::load::{Consumer, TransmissionLine};
use crate::grid::transformer::Transformer;
use crate::grid::{Grid, Resistance, ToJson, Voltage};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::content;
use rocket::serde::json::{json, Json};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response, State};
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

#[get("/")]
fn index() -> String {
    "Yay".to_string()
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GeneratorUpdate {
    id: u32,
    supply: f32,
}

#[post("/produce", format = "application/json", data = "<data>")]
fn produce(grid: &State<Arc<Mutex<Grid>>>, data: Json<GeneratorUpdate>) -> String {
    let id = data.id;
    let supply = data.supply;
    format!("Production of {id} set to {supply}V").to_string()
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ConsumerUpdate {
    id: u32,
    load: f32,
}

#[post("/consume", format = "application/json", data = "<data>")]
fn consume(grid: &State<Arc<Mutex<Grid>>>, data: Json<ConsumerUpdate>) -> String {
    let id = data.id;
    let load = data.load;
    format!("Consumption of {id} set to {load}Ω")
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ConsumerNew {
    resistance: f32,
    transmission_line: u32,
}

#[post("/add_consumer", format = "application/json", data = "<data>")]
fn add_consumer(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<ConsumerNew>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();
    let id = 0;
    content::RawJson(json!({"id":id}).to_string())
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GeneratorNew {
    transmission_line: u32,
    max_voltage: f32,
    frequency: f32,
}

#[post("/add_generator", format = "application/json", data = "<data>")]
fn add_generator(
    grid: &State<Arc<Mutex<Grid>>>,
    data: Json<GeneratorNew>,
) -> content::RawJson<String> {
    let mut g = grid.lock().unwrap();
    let id = 0;
    content::RawJson(json!({"id":id}).to_string())
}

#[post("/info", format = "application/json")]
fn info(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    content::RawJson("{}".parse().unwrap())
}

#[post("/overview", format = "application/json")]
fn overview(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    content::RawJson("{}".parse().unwrap())
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
                grid.update(elapsed_time)
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
                produce,
                consume,
                start,
                info,
                overview,
                add_generator,
                add_consumer
            ],
        )
        .manage(Arc::new(Mutex::new(Grid {
            circuits: vec![],
            frequency: 0.0,
            started: false,
        })))
}
