#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

use crate::models::{NewAdvertisementModel, NewProfileModel, NewUserModel, User, Advertisement};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use pwhash::bcrypt;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::name::NameBuf;
use rocket::http::Header;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;
use rocket::State;
use rocket::{Request, Response};
use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

mod models;
mod schema;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

type TaskQueue = deadqueue::unlimited::Queue<(u64, f32, String)>;

const IDEAL_VOLTAGE: f32 = 230.0;

struct MyInfo {
    price: AtomicU32,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

}

#[get("/")]
async fn index(state: &State<MyInfo>) -> String {
    let res = reqwest::get("http://127.0.0.1:8000/").await.unwrap();
    let body = res.text().await.unwrap();
    let voltage = body.parse::<f32>().unwrap();
    let mut price = state.price.load(Ordering::Relaxed);

    if voltage != 0.0 {
        if voltage > IDEAL_VOLTAGE {
            price = state.price.fetch_add(1, Ordering::Relaxed);
        } else if voltage < IDEAL_VOLTAGE && price > 1 {
            price = state.price.fetch_sub(1, Ordering::Relaxed);
        }
    }

    let open = "{";
    let close = "}";
    format!("{open}\"Voltage\":\"{voltage}\",\"Price\":\"{price}\"{close}")
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
        let index = vec.iter().position(|r| *r == id).unwrap();
        vec.remove(index);
        "true".to_string()
    } else {
        "false".to_string()
    }
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
struct AdvertisementReq<'r>{
    email: &'r str,
    units: f64,
    price: f64,
}

#[post("/advertise", format = "application/json", data = "<new_ad>")]
async fn advertise(new_ad: Json<AdvertisementReq<'_>>) -> Value{

    use self::schema::open_em::advertisements;
    use self::schema::open_em::users::dsl::*;
    let connection = &mut establish_connection();

    let user = users
        .filter(email.eq(new_ad.email))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    let new_advertisement_insert = NewAdvertisementModel {
        seller_id: &user[0].user_id,
        offered_units: &new_ad.units,
        price: &new_ad.price,
    };

    let new_ad_ret = diesel::insert_into(advertisements::table)
        .values(&new_advertisement_insert)
        .returning(Advertisement::as_returning())
        .get_result::<Advertisement>(connection)
        .expect("Error adding new advertisement");

    json!({ "status": "ok", "advertisement_id": new_ad_ret.advertisement_id })

}

// #[derive(Serialize,Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct Offer<'r>{
//     ad_id: &'r Uuid,
//     user_id: &'r Uuid,
//     units: &'r f32,
//     bid: &'r f32,
// }
//
// #[post("/purchase", format = "application/json", data = "<credentials>")]
// async fn purchase() -> Value{
//
//
//
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Credentials<'r> {
    email: &'r str,
    password: &'r str,
}

#[post("/login", format = "application/json", data = "<credentials>")]
async fn login(credentials: Json<Credentials<'_>>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let user = users
        .filter(email.eq(credentials.email))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    let verify = bcrypt::verify(credentials.password, &*user[0].pass_hash);

    json!({ "status": "ok", "verified": verify })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewUser<'r> {
    email: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    password: &'r str,
}

#[post("/register", format = "application/json", data = "<new_user>")]
async fn register(new_user: Json<NewUser<'_>>) -> Value {
    use self::schema::open_em::users;
    use self::schema::open_em::profiles;

    let connection = &mut establish_connection();
    let binding = bcrypt::hash(new_user.password).unwrap();
    let h = binding.as_str();

    let new_user_insert = NewUserModel {
        email: new_user.email,
        pass_hash: h,
    };

    let new_user_ret = diesel::insert_into(users::table)
        .values(&new_user_insert)
        .returning(User::as_returning())
        .get_result::<User>(connection)
        .expect("Error adding new user");

    let new_profile_insert = NewProfileModel {
        user_id: &new_user_ret.user_id,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
    };

    diesel::insert_into(profiles::table)
        .values(&new_profile_insert)
        .execute(connection)
        .expect("Error adding new profile");

    json!({ "status": "ok", "email": new_user.email })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, bid, sell, met, register, login, advertise])
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .manage(Arc::new(TaskQueue::new()))
        .manage(MyInfo {
            price: AtomicU32::new(100),
        })
        .manage(Arc::new(Mutex::new(Vec::<String>::new())))
        .attach(CORS)
}
