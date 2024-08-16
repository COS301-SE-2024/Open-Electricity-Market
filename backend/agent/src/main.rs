#[macro_use]
extern crate rocket;

use std::{env, sync::{Arc, Mutex}, thread, time};

use agent::Agent;
use curve::SineCurve;
use dotenvy::dotenv;
use generator::Generator;
use node::Node;
use rocket::{http::{Header, Method, Status}, Request, Response};
use serde_json::json;
use smart_meter::SmartMeter;
use rocket::response::content;
use rocket::fairing::{Fairing, Info, Kind};

pub mod agent;
pub mod curve;
pub mod generator;
pub mod location;
pub mod net_structs;
pub mod node;
pub mod smart_meter;

const AGENT_SPEED: u64 = 5 * 60;

pub struct CORS;

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

#[post("/stats")]
fn stats() -> content::RawJson<String> {
      content::RawJson(json!({}).to_string())
}

#[launch]
fn rocket() -> _ {

    let mut handels = vec![];
    dotenv().ok();
    let password = env::var("PASSWORD").unwrap();

    for i in 1..15 {
        let password = password.clone();
        let handle = thread::spawn(move || {
            let mut agent = Agent::new(
                String::from(format!("{i}@example.com")),
                password,
                vec![Node::new(
                    SmartMeter::new_acctive(Box::new(SineCurve::new())),
                    Generator::new_acctive(Box::new(SineCurve::new())),
                )],
                0.0,
                Box::new(SineCurve::new()),
            );
            agent.run();
        });
        handels.push(handle);
        thread::sleep(time::Duration::from_secs(1 * AGENT_SPEED));
    }

    rocket::build().attach(CORS)
        .mount(
            "/",
            routes![ 
                stats,
            ],
        ).manage(Arc::new(Mutex::new(Vec::<Agent>::new())))
}

