extern crate reqwest;
use uuid::Uuid;

use std::env;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let procedure = args[1].clone();
    let value = args[2].parse::<u64>().unwrap(); //Roughly how many amps are needed each second by a common south african home
    let desired_price: f32 = 100.0;
    dbg!(args);

    loop {
        let res = reqwest::get("http://127.0.0.1:8001/").await.unwrap();
        let text = res.text().await.unwrap();
        let mut split = text.split("\"Price\":\"");
        split.next();
        let parsel = split.next().unwrap();
        let mut split = parsel.split('"');
        let parsel = split.next().unwrap();
        let price = parsel.parse::<f32>().unwrap();

        let actual;

        if procedure == "consume" {
            actual = ((desired_price / price) * value as f32) as u64;
            println!("Price {price} Desired Price {desired_price} Value {actual}");

            let id = Uuid::new_v4();
            let res = reqwest::get(format!("http://127.0.0.1:8001/bid/{actual}/{price}/{id}"))
                .await
                .unwrap();
            let mut body = res.text().await.unwrap();

            while body != "true" {
                let res = reqwest::get(format!("http://127.0.0.1:8001/met/{id}"))
                    .await
                    .unwrap();
                body = res.text().await.unwrap();
                sleep(Duration::from_millis(1000)).await;
            }
            let res = reqwest::get(format!("http://127.0.0.1:8000/{procedure}/{actual}"))
                .await
                .unwrap();
            let body = res.text().await.unwrap();
            println!("Body:\n{}", body);
        } else {
            actual = ((desired_price / price) * value as f32) as u64;
            //Hence a producer
            let res = reqwest::get(format!("http://127.0.0.1:8001/sell/{actual}"))
                .await
                .unwrap();
            let body = res.text().await.unwrap();
            println!("{body}");
            if body != "There is no demand" && body != "Could not meet demand" {
                let res = reqwest::get(format!("http://127.0.0.1:8000/{procedure}/{body}"))
                    .await
                    .unwrap();
                let body = res.text().await.unwrap();
                println!("Body:\n{}", body);
            }
        }

        sleep(Duration::from_millis(1000)).await;
    }
}
