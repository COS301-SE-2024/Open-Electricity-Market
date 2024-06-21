
pub mod schema;
pub mod models;

pub mod consumer;
pub mod producer;

use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::ops::DerefMut;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::consumer::{Consumer, ConsumerManger, ConsumerNew, ConsumerResponse};
use crate::models::User;
use crate::producer::{Producer, ProducerManager, ProducerNew, ProducerResponse};

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





    let mut consumer_manger = Arc::new(Mutex::new (ConsumerManger {
        map : HashMap::new()
    }));


    let mut producer_manger = Arc::new(Mutex::new (ProducerManager {
        map : HashMap::new()
    }));


    let customer_manger_clone = consumer_manger.clone();
    let producer_manager_clone  = producer_manger.clone();
    tokio::spawn( async move  {

        loop {
            let connection = &mut establish_connection();

            let results: Vec<User> = users.load( connection).expect("Error loading users");
            for user in results {
                println!("{} {}", user.email, user.units_bought);
                println!("-----------");
                //Create new consumer and link it to grid
                let clone = customer_manger_clone.clone();
                let mut cons_manager = clone.lock().await;
                let mut consumer_exits = false;
                match cons_manager.map.get(&user.email) {
                    None => { consumer_exits = false;}
                    Some(_) => { consumer_exits = true;}
                }

                if user.units_bought > 0.0 && !consumer_exits {
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
                        cons_manager.map.insert(user.email.clone(), Consumer { id : json.id,load:1000.0, email: user.email.clone(), last_consumed: 0.0 });
                }

                let clone = producer_manager_clone.clone();
                let mut prod_manager = clone.lock().await;
                let mut producer_exits = false;
                match prod_manager.map.get(&user.email) {
                    None => { producer_exits = false;}
                    Some(_) => { producer_exits = true;}
                }

                if user.units_sold > 0.0 && !producer_exits {
                    let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");

                    let mut data = ProducerNew {
                        transmission_line: 0,
                        max_voltage: 240.0,
                        frequency: 50.0,
                    };

                    grid_url.push_str("/add_generator");
                    println!("{grid_url}");


                    let client = reqwest::Client::new();
                    let resp = client.post(grid_url).json(&data).send().await.expect("Failed to connect to grid");
                    let json = resp.json::<ProducerResponse>().await.expect("Could not decode json");

                    println!("{}",json.id);
                    prod_manager.map.insert(user.email.clone(),Producer { id: 0, email: user.email, on: true });
                }


            };


        }
    });





    let mut start = Instant::now();
    let mut elapsed_time = 0.0;
    let mut total_time = 0.0;
    loop {
        let duration = start.elapsed();
        start = Instant::now();
        elapsed_time = duration.as_secs_f32();
        total_time += elapsed_time;

        let mut manager = consumer_manger.lock().await;

        for (_,consumer) in manager.map.iter_mut() {
            consumer.update(elapsed_time,total_time);
            consumer.sync_grid(producer_manger.clone()).await;
        }

        let mut manager = producer_manger.lock().await;

        for (_,producer) in manager.map.iter_mut() {
            producer.sync_grid().await;
        }

    }


    Ok(())
}