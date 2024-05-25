extern crate reqwest;


use std::env;
use tokio::time::{sleep,Duration};


#[tokio::main]
async fn main()  {
    let args: Vec<String> = env::args().collect();
    let procedure = args[1].clone();
    let mut value  = args[2].parse::<u64>().unwrap(); //Roughtly how many amps are needed each second by a common south african home
    let desired_price: f32 = 100.0;
    dbg!(args);

    loop {

        let res = reqwest::get("http://127.0.0.1:8001/").await.unwrap();
        let body = res.text().await.unwrap();
        let mut split = body.split("'Price':");
        split.next();
        let body = split.next().unwrap().to_string();
        split =  body.split("  ");
        let body = split.next().unwrap().to_string();
        let price = body.parse::<f32>().unwrap();

        let actual ;

        if procedure == "consume" {
            actual = ((desired_price/price)*value as f32 ) as u64;
            println!("Price {price} Desired Price {desired_price} Value {actual}");
        } else {
            let percentage = (price-desired_price)/desired_price;
            let diff = value as f32*percentage;
            if (diff > 0.0) {
                actual = value + diff as u64;
            } else {
                actual = value - (-diff) as u64;
            }
            println!("Price {price} Desired Price {desired_price} Value {actual} Diff {diff}");
        }



        let res = reqwest::get(format!("http://127.0.0.1:8000/{procedure}/{actual}")).await.unwrap();
        let body = res.text().await.unwrap();
        println!("Body:\n{}", body);
        sleep(Duration::from_millis(1000)).await;
    }
}