use crate::establish_connection;
use crate::models::User;
use diesel::RunQueryDsl;
use diesel::{ExpressionMethods, QueryDsl};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

pub struct ProducerManager {
    pub(crate) map: HashMap<String, Producer>,
}

#[derive(Serialize)]
pub struct ProducerNew {
    pub transmission_line: u32,
    pub max_voltage: f32,
    pub frequency: f32,
}

#[derive(Deserialize)]
pub struct ProducerResponse {
    pub id: u32,
}

#[derive(Serialize)]
struct ProducerUpdate {
    id: u32,
    supply: f32,
}

pub struct Producer {
    pub id: u32,
    pub email: String,
    pub on: bool,
}

impl Producer {
    pub async fn sync_grid(&mut self) {
        use super::schema::open_em::users::dsl::*;
        let connection = &mut establish_connection();

        let results: Vec<User> = users
            .filter(email.eq(&self.email))
            .load(connection)
            .expect("Error loading users");

        if results[0].units_sold < 0f64 && self.on {
            self.on = !self.on;
            // Turn off
            let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");
            let client = reqwest::Client::new();

            let data = ProducerUpdate {
                id: self.id,
                supply: 0.0,
            };

            grid_url.push_str("/produce");
            let res = client
                .post(grid_url)
                .json(&data)
                .send()
                .await
                .expect("Could not connect to grid");
            let text = res.text().await.unwrap();
            println!("{text}");
        } else if results[0].units_sold > 0f64 && !self.on {
            self.on = !self.on;
            // Turn on
            let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");
            let client = reqwest::Client::new();

            let data = ProducerUpdate {
                id: self.id,
                supply: 240.0,
            };

            grid_url.push_str("/produce");
            let res = client
                .post(grid_url)
                .json(&data)
                .send()
                .await
                .expect("Could not connect to grid");
            let text = res.text().await.unwrap();
            println!("{text}");
        }
    }
}
