use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::f32::consts::PI;
use std::ops::Sub;
use std::sync::{Arc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rand::seq::SliceRandom;
use tokio::io::unix::AsyncFd;
use tokio::sync::Mutex;
use crate::establish_connection;
use crate::models::User;
use crate::producer::ProducerManager;
use rand::thread_rng;

#[derive(Serialize)]
pub struct ConsumerNew {
    pub resistance: f32,
    pub transmission_line: u32
}

#[derive(Deserialize)]
pub struct ConsumerResponse {
    pub id : u32
}

#[derive(Serialize)]
struct ConsumerUpdate {
    id: u32,
    load: f32,
}

pub struct Consumer {
    pub id :u32,
    pub load: f32,
    pub email : String,
    pub last_consumed: f32,
}

impl Consumer {
    pub fn update(&mut self,elapsed_time : f32,total_time :f32) {
        use super::schema::open_em::users::dsl::*;
        let connection = &mut establish_connection();

        let results: Vec<User> = users.filter(email.eq(&self.email)).load( connection).expect("Error loading users");


        let resistance = 400.0*f32::abs(f32::sin(total_time/800.0*PI)) + 800.0;
        //Assume 240 volts
        let potential_difference = 240.0;
        let current = potential_difference/resistance;
        let power = potential_difference*current;

        let power_per_second = power*elapsed_time;
        let power_per_hour = power_per_second/(60.0*60.0);
        println!("I have {} of which I'll use {}",results[0].units_bought,power_per_hour);
        self.last_consumed = power_per_hour;

        if results[0].units_bought > power_per_hour as f64 {
            self.load = resistance;
        } else  {
            self.load = 0f32;
        }
    }

    pub async fn sync_grid(&self, producer_manager : Arc<Mutex<ProducerManager>> ){
        use super::schema::open_em::users::dsl::*;
        let connection = &mut establish_connection();


        let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");
        let client = reqwest::Client::new();

        let data = ConsumerUpdate {
            id: self.id,
            load: self.load,
        };

        grid_url.push_str("/consume");
        let res  = client.post(grid_url).json(&data).send().await.expect("Could not connect to grid");
        let text = res.text().await.unwrap();
        println!("{text}");

        diesel::update(users).filter(email.eq(&self.email)).set(units_bought.eq(units_bought - self.last_consumed as f64)).execute(connection).expect("Could not update");
        println!("Used {}",self.last_consumed);

        let mut producer_manager = producer_manager.lock().await;

        let mut its = producer_manager.map.iter().collect::<Vec<_>>();
        its.shuffle(&mut thread_rng());

        for (_,producer) in its.iter_mut() {
            if producer.on {
                diesel::update(users).filter(email.eq(&producer.email)).set(units_sold.eq(units_sold - self.last_consumed as f64)).execute(connection).expect("Could not update");
                println!("Lost {}",self.last_consumed);
                break;
            }
        }



    }
}


pub struct ConsumerManger {
    pub(crate) map: HashMap<String,Consumer>,
}