#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

use rocket::State;
use std::fmt::format;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

type TaskQueue = deadqueue::unlimited::Queue<(u64, f32, String)>;

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

    if voltage != 0.0 {
        if voltage > IDEAL_VOLTAGE {
            price = state.price.fetch_add(1, Ordering::Relaxed);
        } else if voltage < IDEAL_VOLTAGE {
            if (price > 1) {
                price = state.price.fetch_sub(1, Ordering::Relaxed);
            }
        }
    }

    let open = "{";
    let close = "}";
    format!("{open} 'Voltage': {voltage}, 'Price':{price}  {close}")
}

#[get("/bid/<amount>/<price>/<id>")]
async fn bid(state: &State<Arc<TaskQueue>>, amount: u64, price: f32, id: String) -> String {
    state.push((amount, price, id));
    let len = state.len();
    format!("Bids {len}")
}

#[get("/sell/<amount>")]
async fn sell(
    bid_queue: &State<Arc<TaskQueue>>,
    sold_list: &State<Arc<Mutex<Vec<String>>>>,
    amount: u64,
) -> String {
    let bid = bid_queue.try_pop();
    match bid {
        None => "There is no demand".to_string(),
        Some((bid_amount, price, id)) => {
            if bid_amount <= amount {
                sold_list.lock().unwrap().push(id);
                format!("{bid_amount}")
            } else {
                bid_queue.push((bid_amount, price, id));
                "Could not meet demand".to_string()
            }
        }
    }
}

#[get("/met/<id>")]
async fn met(sold_list: &State<Arc<Mutex<Vec<String>>>>, id: String) -> String {
    let mut vec = sold_list.lock().unwrap();
    if vec.contains(&id) {
        let index = vec
            .iter()
            .position(|r| r.to_string() == id.to_string())
            .unwrap();
        vec.remove(index);
        "true".to_string()
    } else {
        "false".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, bid, sell, met])
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .manage(Arc::new(TaskQueue::new()))
        .manage(Info {
            price: AtomicU32::new(100),
        })
        .manage(Arc::new(Mutex::new(Vec::<String>::new())))
}
