#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

mod market_interaction;
mod models;
mod node_management;
mod schema;
mod user_management;

use crate::models::User;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::CookieJar;
use rocket::http::{Header, Method, Status};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, Response};
use std::env;
use std::ops::Add;
use uuid::Uuid;

const TRANSACTION_LIFETIME: i64 = 24; // Lifetime in hours
                                      // const TARGET_VOLTAGE: f64 = 240.0;
                                      // Endpoint for current_voltage

const UNIT_PRICE_RATE: f64 = 0.005;
const IMPEDANCE_RATE: f64 = 0.05;
const SUPPLY_DEMAND_RATE: f64 = 0.05;
const TARGET_HISTORY_POINTS: i64 = 100;

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

        response.set_header(Header::new("Access-Control-Allow-Origin", frontend_url));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct Claims {
    message: String,
    user_id: Uuid,
}

fn verify_user(cookie_jar: &CookieJar<'_>) -> Claims {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut response = Claims {
        message: "".to_string(),
        user_id: Uuid::nil(),
    };

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => response.message = "Session ID not found".to_string(),
        Some(cookie) => {
            let cookie_value = cookie.value().parse();
            match cookie_value {
                Ok(cookie_str) => {
                    has_cookie = true;
                    session_id_str = cookie_str;
                }
                Err(_) => {}
            };
        }
    }

    if has_cookie {
        response.message = "No matching user".to_string();
        match users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                response.message = "User found".to_string();
                response.user_id = user.user_id;
            }
            Err(_) => {}
        }
    }

    return response;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GridStats {
    total_impedance: f64,
    total_generation: f64,
    consumer_count: i64,
    producer_count: i64,
    user_count: i64,
}

fn buy_fee_calc(units: f64, price: f64) -> f64 {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut demand = 0f64;

    match buy_orders
        .filter(sought_units.gt(filled_units))
        // .filter(active.eq(true))
        .execute(connection)
    {
        Ok(num_buys) => demand = num_buys as f64,
        Err(_) => {}
    }

    let mut supply = 0f64;
    match sell_orders
        .filter(offered_units.gt(claimed_units))
        // .filter(active.eq(true))
        .execute(connection)
    {
        Ok(num_sells) => supply = num_sells as f64,
        Err(_) => {}
    }

    let mut impedance = 1f64;
    match env::var("GRID_URL") {
        Ok(grid_url) => {
            let client = reqwest::blocking::Client::new();
            match client.post(grid_url + "/stats").send() {
                Ok(response) => match response.json::<GridStats>() {
                    Ok(grid_stats) => impedance = grid_stats.total_impedance,
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    let mut demand_supply_diff = 1f64;
    if supply < demand {
        demand_supply_diff = demand - supply
    }

    return units * price * UNIT_PRICE_RATE
        + (f64::log10(demand_supply_diff) * SUPPLY_DEMAND_RATE)
        + (f64::log10(impedance) * IMPEDANCE_RATE);
}

fn sell_fee_calc(units: f64, price: f64) -> f64 {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut demand = 0f64;

    match buy_orders
        .filter(sought_units.gt(filled_units))
        // .filter(active.eq(true))
        .execute(connection)
    {
        Ok(num_buys) => demand = num_buys as f64,
        Err(_) => {}
    }

    let mut supply = 0f64;
    match sell_orders
        .filter(offered_units.gt(claimed_units))
        // .filter(active.eq(true))
        .execute(connection)
    {
        Ok(num_sells) => supply = num_sells as f64,
        Err(_) => {}
    }

    let mut impedance = 1f64;
    match env::var("GRID_URL") {
        Ok(grid_url) => {
            let client = reqwest::blocking::Client::new();
            match client.post(grid_url + "/stats").send() {
                Ok(response) => match response.json::<GridStats>() {
                    Ok(grid_stats) => impedance = grid_stats.total_impedance,
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    let mut supply_demand_diff = 1f64;
    if demand < supply {
        supply_demand_diff = supply - demand
    }

    return (units * price * UNIT_PRICE_RATE)
        + (f64::log10(supply_demand_diff) * SUPPLY_DEMAND_RATE)
        + (f64::log10(impedance) * IMPEDANCE_RATE);
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
            ],
        )
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .attach(CORS)
}
