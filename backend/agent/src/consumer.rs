use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::f32::consts::PI;


#[derive(Serialize)]
pub struct ConsumerNew {
    pub resistance: f32,
    pub transmission_line: u32
}

#[derive(Deserialize)]
pub struct ConsumerResponse {
    pub id : u32
}

pub struct Consumer {
    pub id :u32,
}

impl Consumer {
    fn update(elapsed_time : f32) {
        let resistance = *400.0+f32::abs(f32::sin(elapsed_time/800.0*PI)) + 800.0;
        //Assume 240 volts
        let potential_difference = 240.0;
        let current = potential_difference/resistance;
        let power = potential_difference*current;



    }

    fn sync_grid(){
        let mut grid_url = env::var("GRID_URL").expect("DATABASE_URL must be set");
        let client = reqwest::blocking::Client::new();
        grid_url.push_str("/consume");
        let res  = client.post(grid_url).json().send().expect("Failed to connect to grid");

    }
}


pub struct ConsumerManger {
    pub(crate) map: HashMap<String,Consumer>,
}