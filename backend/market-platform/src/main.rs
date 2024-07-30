#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

use crate::models::{NewBuyOrderModel, NewNodeModel, NewProfileModel, NewSellOrderModel, NewUserModel, Node, Profile, SellOrder, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use pwhash::bcrypt;
use pwhash::unix::verify;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::name::NameBuf;
use rocket::http::CookieJar;
use rocket::http::{Cookie, Header, Method, Status};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::yansi::Paint;
use rocket::{Request, Response, State};
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

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct SellOrderReq {
//     producer_id: Uuid,
//     units: f64,
//     price: f64,
// }
//
// #[post("/sell_order", format = "application/json", data = "<new_sell_order>")]
// async fn sell_order(new_sell_order: Json<SellOrderReq>, jar: CookieJar<'_>) -> Value {
//     use self::schema::open_em::sell_orders;
//
//     use self::schema::open_em::users::dsl::*;
//
//     let connection = &mut establish_connection();
//
//     let mut message = "Something went wrong";
//
//     let session_cookie = jar.get("session_id");
//
//     let mut has_cookie = false;
//     let mut session_id_str: String = "".to_string();
//     match session_cookie {
//         None => {}
//         Some(cookie) => {
//             has_cookie = true;
//             session_id_str = cookie.value().parse().unwrap();
//         }
//     }
//
//     if has_cookie {
//         let user = users
//             .filter(session_id.eq(session_id_str))
//             .select(User::as_select())
//             .load::<User>(connection)
//             .expect("Error loading users");
//
//         let new_sell_order_insert = NewAdvertisementModel {
//             seller_id: &user[0].user_id,
//             offered_units: &new_sell_order.units,
//             price: &new_sell_order.price,
//         };
//     }
//
//     let new_ad_ret = diesel::insert_into(sell_orders::table)
//         .values(&new_advertisement_insert)
//         .returning(Advertisement::as_returning())
//         .get_result::<Advertisement>(connection)
//         .expect("Error adding new advertisement");
//
//     json!({ "status": "ok", "advertisement_id": new_ad_ret.advertisement_id })
// }

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct GetAdvertisementReq {
//     num_advertisements: i64,
// }
//
// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct RetAdvertisements {
//     advertisement_id: i64,
//     offered_units: f64,
//     price: f64,
// }
//
// #[post("/get_ads", format = "application/json", data = "<ad_req>")]
// async fn get_ads(ad_req: Json<GetAdvertisementReq>) -> Value {
//     use self::schema::open_em::advertisements::dsl::*;
//
//     let advertisements_vec = advertisements
//         .filter(offered_units.gt(0.0))
//         .select(Advertisement::as_select())
//         .order_by(price.asc())
//         .limit(ad_req.num_advertisements)
//         .load::<Advertisement>(&mut establish_connection())
//         .expect("Error loading advertisements");
//
//     let mut advertisements_ret: Vec<RetAdvertisements> = vec![];
//
//     for ad in advertisements_vec {
//         advertisements_ret.push(RetAdvertisements {
//             advertisement_id: ad.advertisement_id,
//             offered_units: ad.offered_units,
//             price: ad.price,
//         });
//     }
//
//     json!({"status": "ok", "advertisements": advertisements_ret})
// }

// #[post("/priceview")]
// async fn priceview() -> Value {
//     use self::schema::open_em::advertisements::dsl::*;
//
//     let price_avg = advertisements
//         .filter(offered_units.gt(0.0))
//         .select(diesel::dsl::sql::<diesel::sql_types::Double>("AVG(price)"))
//         .load::<f64>(&mut establish_connection())
//         .expect("Error loading average price");
//
//     json!({"status":"ok", "price": price_avg[0]})
// }

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct Offer<'r> {
//     ad_id: i64,
//     email: &'r str,
//     units: f64,
// }
//
// #[post("/buy_order", format = "application/json", data = "<new_buy_order>")]
// async fn buy_order(new_buy_order: Json<Offer<'_>>) -> Value {
//
//     use self::schema::open_em::sell_orders::dsl::*;
//     use self::schema::open_em::transactions;
//     use self::schema::open_em::users::dsl::*;
//
//     let connection = &mut establish_connection();
//
//     let user = users
//         .filter(email.eq(new_buy_order.email))
//         .select(User::as_select())
//         .load::<User>(connection)
//         .expect("Error loading users");
//
//     let advertisements_vec = advertisements
//         .filter(offered_units.gt(0.0))
//         .select(Advertisement::as_select())
//         .order_by(price.asc())
//         .load::<Advertisement>(connection)
//         .expect("Error loading advertisements");
//
//     let mut purchase = false;
//
//     if advertisements_vec[0].offered_units >= new_buy_order.units {
//         let new_transaction_insert = NewTransactionModel {
//             buyer_id: &user[0].user_id,
//             advertisement_id: &new_buy_order.ad_id,
//             bought_units: &new_buy_order.units,
//         };
//
//         diesel::insert_into(transactions::table)
//             .values(&new_transaction_insert)
//             .execute(connection)
//             .expect("Error adding new transaction");
//
//         purchase = true;
//     }
//
//     json!({"status": "ok", "purchase": purchase})
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserDetails {
    email: String,
    credit: f64,
    first_name: String,
    last_name: String,
}

#[post("/user_details", format = "application/json")]
async fn user_details(cookie_jar: &CookieJar<'_>) -> Value {

    use self::schema::open_em::users::dsl::*;
    use self::schema::open_em::profiles::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => {}
        Some(cookie) => {
            has_cookie = true;
            session_id_str = cookie.value().parse().unwrap();
        }
    }

    let mut data = UserDetails{
        email: "".to_string(),
        credit: 0.0,
        first_name: "".to_string(),
        last_name: "".to_string(),
    };

    if has_cookie {

        let user_ret = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection)
            .expect("User does not exist");

        let temp_user_id = user_ret[0].user_id.clone();
        let user_email = user_ret[0].email.clone();

        let profile_ret = profiles
            .filter(profile_user_id.eq(temp_user_id))
            .select(Profile::as_select())
            .load::<Profile>(connection)
            .expect("Could not find profile");

        let user_first_name = profile_ret[0].first_name.clone();
        let user_last_name = profile_ret[0].last_name.clone();

        data = UserDetails{
            email: user_email,
            credit: user_ret[0].credit,
            first_name: user_first_name,
            last_name: user_last_name,
        };

        message = "User details successfully retrieved"

    }

    json!({"status": "ok", "message": message, "data": data})
}

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct NodeDetails{
//
// }
//
// #[derive(Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// struct NodeDetailsReq{
//
// }
//
// #[post("/node_details", format = "application/json", data = "<node_details_req>")]
// async fn node_details(node_details_req: NodeDetailsReq, cookie_jar: &CookieJar<'_>) -> Value {
//
//
//     json!({"status": "ok", "message": message, "data": })
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GetNodesReq {
    limit: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct ShortNodeRet {
    node_id: String,
    name: String,
}

#[post("/get_nodes", format = "application/json", data = "<get_nodes_req>")]
async fn get_nodes(get_nodes_req: Json<GetNodesReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => {}
        Some(cookie) => {
            has_cookie = true;
            session_id_str = cookie.value().parse().unwrap();
        }
    }

    let mut node_list: Vec<ShortNodeRet> = vec![];

    if has_cookie {
        let user_ret = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection)
            .expect("User does not exist");

        let nodes_vec = nodes
            .filter(node_owner.eq(user_ret[0].user_id))
            .select(Node::as_select())
            .limit(get_nodes_req.limit)
            .load::<Node>(connection)
            .expect("Could not get nodes");

        for node in nodes_vec {
            node_list.push(ShortNodeRet {
                node_id: node.node_id.to_string(),
                name: node.name,
            })
        }
        message = "List of nodes successfully retrieved"
    }

    json!({"status": "ok", "message": message, "data": node_list})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddNodeReq<'r> {
    name: &'r str,
    location_x: f64,
    location_y: f64,
}

#[post("/add_node", format = "application/json", data = "<add_node_req>")]
async fn add_node(add_node_req: Json<AddNodeReq<'_>>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::nodes;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => {}
        Some(cookie) => {
            has_cookie = true;
            session_id_str = cookie.value().parse().unwrap();
        }
    }

    if has_cookie {
        let user = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection)
            .expect("User does not exist");

        let new_node_insert = NewNodeModel {
            node_owner: user[0].user_id,
            location_x: add_node_req.location_x,
            location_y: add_node_req.location_y,
            name: add_node_req.name,
        };

        diesel::insert_into(nodes::table)
            .values(&new_node_insert)
            .execute(connection)
            .expect("Node Add Failed");

        message = "New Node Added"
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddFundsReq {
    funds: f64,
}

#[post("/add_funds", format = "application/json", data = "<add_funds_req>")]
async fn add_funds(add_funds_req: Json<AddFundsReq>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => {}
        Some(cookie) => {
            has_cookie = true;
            session_id_str = cookie.value().parse().unwrap();
        }
    }

    if has_cookie {
        if add_funds_req.funds > 0f64 {
            diesel::update(users)
                .filter(session_id.eq(session_id_str))
                .set(credit.eq(credit + add_funds_req.funds))
                .execute(connection)
                .expect("Funds update failed");
            message = "Funds added";
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RemoveFundsReq {
    funds: f64,
}

#[post(
    "/remove_funds",
    format = "application/json",
    data = "<remove_funds_req>"
)]
async fn remove_funds(remove_funds_req: Json<RemoveFundsReq>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => {}
        Some(cookie) => {
            has_cookie = true;
            session_id_str = cookie.value().parse().unwrap();
        }
    }

    if has_cookie {
        let user = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection)
            .expect("User does not exist");

        if remove_funds_req.funds > 0f64 && user[0].credit >= remove_funds_req.funds {
            diesel::update(users)
                .filter(user_id.eq(user[0].user_id))
                .set(credit.eq(credit - remove_funds_req.funds))
                .execute(connection)
                .expect("Funds update failed");
            message = "Funds removed";
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Credentials<'r> {
    email: &'r str,
    password: &'r str,
}

#[post("/login", format = "application/json", data = "<credentials>")]
async fn login(credentials: Json<Credentials<'_>>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let user = users
        .filter(email.eq(credentials.email))
        .select(User::as_select())
        .load::<User>(connection)
        .expect("Error loading users");

    let verify = bcrypt::verify(credentials.password, &*user[0].pass_hash);

    if verify {
        let h =
            bcrypt::hash(user[0].user_id.to_string() + &*chrono::Utc::now().to_string()).unwrap();
        let h2 = h.clone();
        diesel::update(users)
            .filter(email.eq(credentials.email))
            .set(session_id.eq(h2))
            .execute(connection)
            .expect("Couldn't update session id");
        jar.add(("session_id", h));
        message = "User logged in"
    }

    json!({ "status": "ok", "message": message })
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
async fn register(new_user: Json<NewUser<'_>>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::profiles;
    use self::schema::open_em::users;
    use self::schema::open_em::users::dsl::*;

    let mut message = "Something went wrong";

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

    let binding_2 =
        bcrypt::hash(new_user_ret.user_id.to_string() + &*new_user_ret.created_at.to_string())
            .unwrap();
    let binding_3 = binding_2.clone();

    diesel::update(users)
        .filter(user_id.eq(new_user_ret.user_id))
        .set(session_id.eq(binding_3))
        .execute(connection)
        .expect("Error making session id");

    jar.add(("session_id", binding_2));

    let new_profile_insert = NewProfileModel {
        profile_user_id: &new_user_ret.user_id,
        first_name: new_user.first_name,
        last_name: new_user.last_name,
    };

    diesel::insert_into(profiles::table)
        .values(&new_profile_insert)
        .execute(connection)
        .expect("Error adding new profile");

    message = "New user added";

    json!({ "status": "ok", "message": message })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                register,
                login,
                add_funds,
                remove_funds,
                add_node,
                get_nodes,
                user_details,
                // sell_order,
                // buy_order,
                // priceview,
            ],
        )
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .attach(CORS)
}
