extern crate reqwest;


use std::env;
use tokio::time::{sleep,Duration};


#[tokio::main]
async fn main()  {
    let args: Vec<String> = env::args().collect();
    let procedure = args[1].clone();
    let value  = args[2].parse::<u64>().unwrap(); //Roughtly how many amps are needed each second by a common south african home

    dbg!(args);

    loop {
        let res = reqwest::get(format!("http://127.0.0.1:8000/{procedure}/{value}")).await.unwrap();
        let body = res.text().await.unwrap();
        println!("Body:\n{}", body);
        sleep(Duration::from_millis(1000)).await;
    }
}