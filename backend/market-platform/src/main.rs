#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

mod market_interaction;
mod models;
mod node_management;
mod schema;
mod user_management;

use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use std::env;

const TRANSACTION_LIFETIME: i64 = 24; // Lifetime in hours
                                      // const TARGET_VOLTAGE: f64 = 240.0;
                                      // Endpoint for current_voltage

const UNIT_PRICE_RATE: f64 = 0.005;
const IMPEDANCE_RATE: f64 = 0.05;
const SUPPLY_DEMAND_RATE: f64 = 0.05;
const TARGET_HISTORY_POINTS: i64 = 100;

const TOKEN_EXPIRATION: Duration = Duration::minutes(15);

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
            response.set_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        }
        dotenv().ok();
        let frontend_url = env::var("FRONTEND_URL").unwrap();

        response.set_header(Header::new("Access-Control-Allow-Origin","*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                user_management::register,
                user_management::login,
                user_management::add_funds,
                user_management::remove_funds,
                user_management::user_details,
                user_management::remove_account,
                node_management::add_node,
                node_management::get_nodes,
                node_management::node_details,
                node_management::update_consumed_units,
                node_management::update_produced_units,
                node_management::remove_node,
                market_interaction::price_view,
                market_interaction::price_history,
                market_interaction::sell_order,
                market_interaction::buy_order,
                market_interaction::list_open_sells,
                market_interaction::list_open_buys,
                market_interaction::estimate_buy_fee,
                market_interaction::estimate_sell_fee,
                market_interaction::all_open_buy,
                market_interaction::all_open_sell,
            ],
        )
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .attach(CORS)
}
