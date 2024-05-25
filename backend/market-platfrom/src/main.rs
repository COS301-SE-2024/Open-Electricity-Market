#[macro_use] extern crate rocket;
extern crate reqwest;

use std::fmt::format;
use std::sync::atomic::{AtomicU32, Ordering};
use rocket::State;

const IDEAL_VOLTAGE: f32 = 230.0;
struct Info {
    price: AtomicU32,
}

#[get("/")]
async fn index(state: &State<Info>) -> String {
    let res = reqwest::get("http://127.0.0.1:8000/").await.unwrap();
    let body = res.text().await.unwrap();
    let voltage = body.parse::<f32>().unwrap();
    let mut price = state.price.load(Ordering::Relaxed);

    if voltage > IDEAL_VOLTAGE {
        price = state.price.fetch_add(1,Ordering::Relaxed);
    } else if voltage < crate::IDEAL_VOLTAGE {
        price = state.price.fetch_sub(1,Ordering::Relaxed);
    }

    let open = "{";
    let close  = "}";
    format!("'{open} 'Voltage': {voltage}, 'Price':{price}  {close}'")
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .manage(Info {price : AtomicU32::new(100)})
}