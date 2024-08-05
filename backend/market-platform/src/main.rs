#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

use crate::models::{
    BuyOrder, NewBuyOrder, NewNodeModel, NewProfileModel, NewSellOrder, NewTransaction,
    NewUserModel, Node, Profile, SellOrder, Transaction, User,
};
use crate::schema::open_em::users::dsl::users;
use crate::schema::open_em::users::session_id;
use chrono::{Duration, Utc};
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
use uuid::{Error, Uuid};

const TRANSACTION_LIFETIME: i64 = 24; // Lifetime in hours
const FRONTEND_URL: &str = "http://localhost:5173";

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
            response.set_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", FRONTEND_URL));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

struct Claims {
    message: String,
    user_id: Uuid,
}

fn verify_user(cookie_jar: &CookieJar<'_>) -> Claims {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut response = Claims {
        message: "".to_string(),
        user_id: Uuid::nil(),
    };

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => response.message = "Session ID not found".to_string(),
        Some(cookie) => {
            let cookie_value = cookie.value().parse();
            match cookie_value {
                Ok(cookie_str) => {
                    has_cookie = true;
                    session_id_str = cookie_str;
                }
                Err(_) => {}
            };
        }
    }

    if has_cookie {
        response.message = "No matching user".to_string();
        match users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .first(connection) {
            Ok(user) => {
                response.message = "User found".to_string();
                response.user_id = user.user_id;
            }
            Err(_) => {}
        }
    }

    return response
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RemoveNode {
    node_id: String,
}

#[post(
    "/remove_node",
    format = "application/json",
    data = "<remove_node_request>"
)]
async fn remove_node(remove_node_request: Json<RemoveNode>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::nodes::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match Uuid::parse_str(&*remove_node_request.node_id) {
            Ok(request_node_id) => {
                message = "No matching node".to_string();
                match diesel::update(nodes)
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_id.eq(request_node_id))
                    .set(node_active.eq(false))
                    .execute(connection)
                {
                    Ok(_) => message = "Node successfully removed".to_string(),
                    Err(_) => message = "Something went wrong.".to_string(),
                }
            }
            Err(_) => message = "Invalid Node ID".to_string(),
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct OpenBuy {
    order_id: i64,
    sought_units: f64,
    filled_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_buys")]
async fn list_open_buys(cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::buy_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match buy_orders
            .filter(buyer_id.eq(claims.user_id))
            .filter(sought_units.gt(filled_units))
            .select(BuyOrder::as_select())
            .load::<BuyOrder>(connection)
        {
            Ok(order_vec) => {
                message = "No open buy orders".to_string();
                if order_vec.len() > 0 {
                    message = "Successfully retrieved open buy orders".to_string();
                    for order in order_vec {
                        data.push(OpenBuy {
                            order_id: order.buy_order_id,
                            sought_units: order.sought_units,
                            filled_units: order.filled_units,
                            max_price: order.max_price,
                            min_price: order.min_price,
                            last_transacted_price: 0f64,
                        })
                    }
                }
            }
            Err(_) => message = "Something went wrong.".to_string(),
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct OpenSell {
    order_id: i64,
    offered_units: f64,
    claimed_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_sells")]
async fn list_open_sells(cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match sell_orders
            .filter(seller_id.eq(claims.user_id))
            .filter(offered_units.gt(claimed_units))
            .select(SellOrder::as_select())
            .load::<SellOrder>(connection)
        {
            Ok(order_vec) => {
                message = "No open sell orders".to_string();
                if order_vec.len() > 0 {
                    message = "Successfully retrieved open sell orders".to_string();
                    for order in order_vec {
                        data.push(OpenSell {
                            order_id: order.sell_order_id,
                            offered_units: order.offered_units,
                            claimed_units: order.claimed_units,
                            max_price: order.max_price,
                            min_price: order.min_price,
                            last_transacted_price: 0f64,
                        })
                    }
                }
            }
            Err(_) => message = "Something went wrong.".to_string(),
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct BuyOrderRequest {
    node_id: String,
    max_price: f64,
    min_price: f64,
    units: f64,
}

#[post(
    "/buy_order",
    format = "application/json",
    data = "<buy_order_request>"
)]
async fn buy_order(buy_order_request: Json<BuyOrderRequest>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;
    use self::schema::open_em::transactions::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        message = "Invalid Node ID".to_string();
        match Uuid::parse_str(&*buy_order_request.node_id.clone()) {
            Ok(request_node_id) => {
                message = "No matching node".to_string();
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .select(Node::as_select())
                    .first(connection)
                {
                    Ok(node) => {
                        let new_buy_order = NewBuyOrder {
                            buyer_id: claims.user_id,
                            consumer_id: node.node_id,
                            sought_units: buy_order_request.units,
                            max_price: buy_order_request.max_price,
                            min_price: buy_order_request.min_price,
                        };
                        message = "Failed to add new buy order".to_string();
                        match diesel::insert_into(buy_orders)
                            .values(new_buy_order)
                            .returning(BuyOrder::as_returning())
                            .get_result(connection)
                        {
                            Ok(mut order) => {
                                message = "Buy order created successfully.".to_string();
                                match sell_orders
                                    .filter(offered_units.gt(claimed_units))
                                    .filter(
                                        schema::open_em::sell_orders::min_price.le(order.max_price),
                                    )
                                    .filter(
                                        schema::open_em::sell_orders::min_price.ge(order.min_price),
                                    )
                                    .filter(seller_id.ne(order.buyer_id))
                                    .filter(producer_id.ne(order.consumer_id))
                                    .order_by(schema::open_em::sell_orders::created_at.asc())
                                    .select(SellOrder::as_select())
                                    .load::<SellOrder>(connection)
                                {
                                    Ok(sell_order_vec) => {
                                        message = "Buy order created successfully. Pending match"
                                            .to_string();
                                        let mut order_match = false;
                                        for s_order in sell_order_vec {
                                            let transaction_units: f64;
                                            if s_order.offered_units - s_order.claimed_units
                                                > order.sought_units - order.filled_units
                                            {
                                                transaction_units =
                                                    order.sought_units - order.filled_units;
                                            } else {
                                                transaction_units =
                                                    s_order.offered_units - s_order.claimed_units;
                                            }
                                            let transaction_price = s_order.min_price; // Will be based on the direction the market needs to move for grid stability
                                            let new_transaction = NewTransaction {
                                                sell_order_id: s_order.sell_order_id,
                                                buy_order_id: order.buy_order_id,
                                                transacted_units: transaction_units,
                                                transacted_price: transaction_price,
                                            };
                                            match diesel::insert_into(transactions)
                                                .values(new_transaction)
                                                .returning(Transaction::as_returning())
                                                .get_result(connection)
                                            {
                                                Ok(transaction) => {
                                                    order_match = true;
                                                    order.filled_units +=
                                                        transaction.transacted_units;
                                                }
                                                Err(error) => {
                                                    message = error.to_string().clone();
                                                }
                                            }
                                            if order.filled_units == order.sought_units {
                                                break;
                                            }
                                        }
                                        if order_match {
                                            message =
                                                "Buy order created successfully. Order matched"
                                                    .to_string()
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SellOrderRequest {
    node_id: String,
    max_price: f64,
    min_price: f64,
    units: f64,
}

#[post(
    "/sell_order",
    format = "application/json",
    data = "<sell_order_request>"
)]
async fn sell_order(
    sell_order_request: Json<SellOrderRequest>,
    cookie_jar: &CookieJar<'_>,
) -> Value {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;
    use self::schema::open_em::transactions::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        message = "Invalid Node ID".to_string();
        match Uuid::parse_str(&*sell_order_request.node_id.clone()) {
            Ok(request_node_id) => {
                message = "No matching node".to_string();
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .select(Node::as_select())
                    .first(connection)
                {
                    Ok(node) => {
                        let new_sell_order = NewSellOrder {
                            seller_id: claims.user_id,
                            offered_units: sell_order_request.units,
                            max_price: sell_order_request.max_price,
                            min_price: sell_order_request.min_price,
                            producer_id: node.node_id,
                        };
                        message = "Failed to add new sell order".to_string();
                        match diesel::insert_into(sell_orders)
                            .values(new_sell_order)
                            .returning(SellOrder::as_returning())
                            .get_result(connection)
                        {
                            Ok(mut order) => {
                                message = "Sell order created successfully".to_string();
                                match buy_orders
                                    .filter(sought_units.gt(filled_units))
                                    .filter(
                                        schema::open_em::buy_orders::min_price.le(order.max_price),
                                    )
                                    .filter(
                                        schema::open_em::buy_orders::min_price.ge(order.min_price),
                                    )
                                    .filter(buyer_id.ne(order.seller_id))
                                    .filter(consumer_id.ne(order.producer_id))
                                    .order_by(schema::open_em::buy_orders::created_at.asc())
                                    .select(BuyOrder::as_select())
                                    .load::<BuyOrder>(connection)
                                {
                                    Ok(buy_order_vec) => {
                                        message = "Sell order created successfully. Pending match"
                                            .to_string();
                                        let mut order_match = false;
                                        for b_order in buy_order_vec {
                                            let transaction_units: f64;
                                            if b_order.sought_units - b_order.filled_units
                                                > order.offered_units - order.claimed_units
                                            {
                                                transaction_units =
                                                    order.offered_units - order.claimed_units;
                                            } else {
                                                transaction_units =
                                                    b_order.sought_units - b_order.filled_units;
                                            }
                                            let transaction_price = b_order.min_price; // Will be based on the direction the market needs to move for grid stability
                                            let new_transaction = NewTransaction {
                                                buy_order_id: b_order.buy_order_id,
                                                sell_order_id: order.sell_order_id,
                                                transacted_units: transaction_units,
                                                transacted_price: transaction_price,
                                            };
                                            match diesel::insert_into(transactions)
                                                .values(new_transaction)
                                                .returning(Transaction::as_returning())
                                                .get_result(connection)
                                            {
                                                Ok(transaction) => {
                                                    order_match = true;
                                                    order.claimed_units +=
                                                        transaction.transacted_units;
                                                }
                                                Err(error) => {
                                                    message = error.to_string().clone();
                                                }
                                            }
                                            if order.claimed_units == order.offered_units {
                                                break;
                                            }
                                        }
                                        if order_match {
                                            message =
                                                "Sell order created successfully. Order matched"
                                                    .to_string()
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[post("/remove_account")]
async fn remove_account(cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message: String;

    let claims = verify_user(cookie_jar);
    message = claims.message.clone();

    if claims.message == "User found" {
        message = "Something went wrong".to_string();
        match diesel::update(users)
            .filter(user_id.eq(claims.user_id))
            .set(active.eq(false))
            .execute(connection)
        {
            Ok(_) => {
                message = "Account successfully deleted".to_string();
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Price {
    price: f64,
}

#[post("/price_view")]
async fn price_view() -> Value {
    use self::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong".to_string();
    let mut data = Price { price: 0f64 };

    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

    match transactions
        .filter(created_at.gt(timestamp))
        .order_by(created_at.desc())
        .select(Transaction::as_select())
        .load::<Transaction>(connection) {
        Ok(transactions_vec) => {
            if transactions_vec.len() > 0 {
                message = "Successfully retrieved price".to_string();
                data = Price {
                    price: transactions_vec[0].transacted_price,
                };
            }
        }
        Err(_) => {}
    }

    json!({"status": "ok", "message":message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserDetails {
    email: String,
    credit: f64,
    first_name: String,
    last_name: String,
}

#[post("/user_details")]
async fn user_details(cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::profiles::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut data = UserDetails {
        email: "".to_string(),
        credit: 0.0,
        first_name: "".to_string(),
        last_name: "".to_string(),
    };

    let mut message = claims.message.clone();

    if claims.message == "User found" {
        match users
            .filter(user_id.eq(claims.user_id))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                message = "No matching user profile".to_string();
                match profiles
                    .filter(profile_user_id.eq(user.user_id))
                    .select(Profile::as_select())
                    .first(connection)
                {
                    Ok(profile) => {
                        data = UserDetails {
                            email: user.email.clone(),
                            credit: user.credit,
                            first_name: profile.first_name.clone(),
                            last_name: profile.last_name.clone(),
                        };
                        message = "User details successfully retrieved".to_string();
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NodeDetails {
    node_id: String,
    name: String,
    location_x: f64,
    location_y: f64,
    units_to_produce: f64,
    units_to_consume: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NodeDetailsReq {
    node_id: String,
}

#[post(
    "/node_details",
    format = "application/json",
    data = "<node_details_request>"
)]
async fn node_details(
    node_details_request: Json<NodeDetailsReq>,
    cookie_jar: &CookieJar<'_>,
) -> Value {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;
    use self::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut data = NodeDetails {
        node_id: "".to_string(),
        name: "".to_string(),
        location_x: 0.0,
        location_y: 0.0,
        units_to_produce: 0.0,
        units_to_consume: 0.0,
    };

    let mut message = claims.message.clone();
    if claims.message == "User found" {
        message = "Invalid Node ID".to_string();
        match Uuid::parse_str(&*node_details_request.node_id) {
            Ok(request_node_id) => {
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_active.eq(true))
                    .select(Node::as_select())
                    .load::<Node>(connection)
                {
                    Ok(node_vec) => {
                        message = "No matching node".to_string();
                        if node_vec.len() > 0 {
                            message = "Node details retrieved succesfully".to_string();
                            data.node_id = String::from(node_vec[0].node_id);
                            data.name = node_vec[0].name.clone();
                            data.location_x = node_vec[0].location_x;
                            data.location_y = node_vec[0].location_y;

                            let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                            match transactions
                                .inner_join(sell_orders)
                                .filter(producer_id.eq(node_vec[0].node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                                    "SUM(transacted_units - units_produced)",
                                ))
                                .load::<f64>(connection)
                                // .select((Transaction::as_select(), SellOrder::as_select()))
                                // .load::<(Transaction, SellOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    if result_vec.len() > 0 {
                                        data.units_to_produce = result_vec[0];
                                    }
                                    /*message = result_vec.len().to_string();
                                    for row in result_vec {
                                        data.units_to_produce += row.0.transacted_units - row.0.units_produced;
                                    }*/
                                }
                                Err(_) => {
                                    message = "Units produced error".to_string();
                                }
                            };

                            match transactions
                                .inner_join(
                                    buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                                        .eq(schema::open_em::transactions::dsl::buy_order_id)),
                                )
                                .filter(consumer_id.eq(node_vec[0].node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                                    "SUM(transacted_units - units_consumed)",
                                ))
                                .load::<f64>(connection)
                                // .select((Transaction::as_select(), BuyOrder::as_select()))
                                // .load::<(Transaction, BuyOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    if result_vec.len() > 0 {
                                        data.units_to_consume = result_vec[0];
                                    }
                                    /*message = result_vec.len().to_string();
                                    for row in result_vec {
                                        data.units_to_consume += row.0.transacted_units - row.0.units_consumed;
                                    }*/
                                }
                                Err(_) => {}
                            };
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

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

#[post(
    "/get_nodes",
    format = "application/json",
    data = "<get_nodes_request>"
)]
async fn get_nodes(get_nodes_request: Json<GetNodesReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    let mut node_list: Vec<ShortNodeRet> = vec![];

    if claims.message == "User found" {
        match nodes
            .filter(node_owner.eq(claims.user_id))
            .filter(node_active.eq(true))
            .select(Node::as_select())
            .limit(get_nodes_request.limit)
            .load::<Node>(connection)
        {
            Ok(node_vec) => {
                for node in node_vec {
                    node_list.push(ShortNodeRet {
                        node_id: node.node_id.to_string(),
                        name: node.name,
                    })
                }
                message = "List of nodes successfully retrieved".to_string()
            }
            Err(_) => {}
        }
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

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.message == "User found" {
        let new_node_insert = NewNodeModel {
            node_owner: claims.user_id,
            location_x: add_node_req.location_x,
            location_y: add_node_req.location_y,
            name: add_node_req.name,
        };

        match diesel::insert_into(nodes::table)
            .values(&new_node_insert)
            .execute(connection)
        {
            Ok(_) => message = "New Node Added".to_string(),
            Err(_) => message = "Something went wrong".to_string(),
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddFundsReq {
    funds: f64,
}

#[post("/add_funds", format = "application/json", data = "<add_funds_req>")]
async fn add_funds(add_funds_req: Json<AddFundsReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.user_id != Uuid::nil() {
        if add_funds_req.funds > 0f64 {
            match diesel::update(users)
                .filter(user_id.eq(claims.user_id))
                .set(credit.eq(credit + add_funds_req.funds))
                .execute(connection)
            {
                Ok(_) => message = "Funds added".to_string(),
                Err(_) => message = "Something went wrong.".to_string(),
            }
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
async fn remove_funds(remove_funds_req: Json<RemoveFundsReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.user_id != Uuid::nil() {
        match users
            .filter(user_id.eq(claims.user_id))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                message = "Insufficient funds".to_string();
                if remove_funds_req.funds > 0f64 && user.credit >= remove_funds_req.funds {
                    message = "Failed to remove funds".to_string();
                    match diesel::update(users)
                        .filter(user_id.eq(user.user_id))
                        .set(credit.eq(credit - remove_funds_req.funds))
                        .execute(connection)
                    {
                        Ok(_) => message = "Funds removed".to_string(),
                        Err(_) => message = "Something went wrong.".to_string(),
                    }
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Credentials {
    email: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<credentials>")]
async fn login(credentials: Json<Credentials>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong".to_string();

    let mut ret_session_id = "".to_string();

    match users
        .filter(email.eq(credentials.email.clone()))
        .select(User::as_select())
        .first(connection)
    {
        Ok(user) => {
            message = "Invalid password".to_string();
            let verify = bcrypt::verify(credentials.password.clone(), &*user.pass_hash);
            if verify {
                match bcrypt::hash(user.user_id.to_string() + &*chrono::Utc::now().to_string()) {
                    Ok(hash) => {
                        match diesel::update(users)
                            .filter(email.eq(credentials.email.clone()))
                            .set(session_id.eq(hash.clone()))
                            .execute(connection)
                        {
                            Ok(_) => {
                                message = "User logged in".to_string();
                                ret_session_id = hash.clone();
                                jar.add(Cookie::build(("session_id", hash)).path("/"))
                            }
                            Err(_) => message = "Failed to update session id".to_string(),
                        }
                    }
                    Err(_) => {}
                }
            }
        }
        Err(_) => message = "User does not exist".to_string(),
    }

    json!({ "status": "ok", "message": message, "data": { "session_id": ret_session_id}})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewUserReq {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[post("/register", format = "application/json", data = "<new_user>")]
async fn register(new_user: Json<NewUserReq>, jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::profiles;
    use self::schema::open_em::users;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let binding = bcrypt::hash(new_user.password.clone()).unwrap();

    let new_user_insert = NewUserModel {
        email: new_user.email.clone(),
        pass_hash: binding,
    };

    let mut ret_session_id = "".to_string();

    let mut message = "Failed to create new user";
    match diesel::insert_into(users::table)
        .values(&new_user_insert)
        .returning(User::as_returning())
        .get_result::<User>(connection)
    {
        Ok(user) => {
            message = "Failed to update Session ID";
            let binding_2 =
                bcrypt::hash(user.user_id.to_string() + &*user.created_at.to_string()).unwrap();
            match diesel::update(users)
                .filter(user_id.eq(user.user_id))
                .set(session_id.eq(binding_2))
                .returning(User::as_returning())
                .get_result(connection)
            {
                Ok(user_up) => {
                    message = "Failed to add user profile";
                    let new_profile_insert = NewProfileModel {
                        profile_user_id: user.user_id,
                        first_name: new_user.first_name.clone(),
                        last_name: new_user.last_name.clone(),
                    };
                    match diesel::insert_into(profiles::table)
                        .values(&new_profile_insert)
                        .execute(connection)
                    {
                        Ok(_) => {
                            message = "New user added";
                            ret_session_id = user_up.session_id.clone().unwrap();
                            jar.add(
                                Cookie::build(("session_id", user_up.session_id.unwrap()))
                                    .path("/"),
                            );
                        }
                        Err(_) => {}
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    json!({ "status": "ok", "message": message, "data": {"session_id": ret_session_id}})
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
                node_details,
                price_view,
                remove_account,
                sell_order,
                buy_order,
                list_open_sells,
                list_open_buys,
                remove_node,
            ],
        )
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .attach(CORS)
}
