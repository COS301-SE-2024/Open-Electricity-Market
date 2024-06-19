
pub mod schema;
pub mod models;

pub mod consumer;

use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::consumer::{Consumer, ConsumerManger, ConsumerNew, ConsumerResponse};
use crate::models::User;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use self::schema::open_em::users::dsl::*;
    dotenv().ok();

    let connection = &mut establish_connection();
    let results: Vec<User> = users.load(connection).expect("Error loading users");

    println!("Displaying {} users", results.len());



    let mut consumer_manger = Arc::new(Mutex::new (ConsumerManger {
        map : HashMap::new()
    }));



    for user in results {
        println!("{} {}", user.email, user.units_bought);
        println!("-----------");
        //Create new consumer and link it to grid
        if user.units_bought > 0.0 {
            let clone = consumer_manger.clone();
            tokio::spawn(async move {
                let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");

                let mut data = ConsumerNew {
                    resistance: 1000.0,
                    transmission_line: 1,
                };

                grid_url.push_str("/add_consumer");
                println!("{grid_url}");

                let client = reqwest::Client::new();
                let resp = client.post(grid_url).json(&data).send().await.expect("Failed to connect to grid");
                let json = resp.json::<ConsumerResponse>().await.expect("Could not decode json");

                println!("{}",json.id);
                let mut manager = clone.lock().await;
                manager.map.insert(user.email, Consumer { id : json.id });
            });
        }
    };

    loop {

    }


    Ok(())
}