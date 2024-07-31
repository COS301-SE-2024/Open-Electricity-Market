#[macro_use]
extern crate rocket;
extern crate deadqueue;
extern crate reqwest;

use crate::models::{
    BuyOrder, NewBuyOrder, NewNodeModel, NewProfileModel, NewSellOrder, NewUserModel, Node,
    Profile, SellOrder, Transaction, User,
};
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

#[post("/list_open_buys")]
async fn list_open_buys(cookie_jar: &CookieJar<'_>) -> Value {
    use self::schema::open_em::users::dsl::*;

    let mut message = "Something went wrong";

    json!({"status": "ok", "message": message})
}

#[post("/list_open_sells")]
async fn list_open_sells(cookie_jar: &CookieJar<'_>) -> Value {
    let mut message = "stub";

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct BuyOrderRequest {
    node_id: String,
    price: f64,
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
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => message = "Session ID not found",
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
        let user_res = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection);

        match user_res {
            Ok(user_vec) => {
                message = "No matching user";
                if user_vec.len() > 0 {
                    message = "No matching node";
                    let node_res = nodes
                        .filter(node_owner.eq(user_vec[0].user_id))
                        .select(Node::as_select())
                        .load::<Node>(connection);
                    match node_res {
                        Ok(node_vec) => {
                            if node_vec.len() > 0 {
                                let new_buy_order = NewBuyOrder {
                                    buyer_id: user_vec[0].user_id,
                                    consumer_id: node_vec[0].node_id,
                                    sought_units: buy_order_request.units,
                                    price: buy_order_request.price,
                                };
                                message = "Failed to add new buy order";
                                match diesel::insert_into(buy_orders)
                                    .values(new_buy_order)
                                    .returning(BuyOrder::as_returning())
                                    .get_result(connection)
                                {
                                    Ok(order) => message = "Buy order created successfully.",
                                    Err(_) => {}
                                };
                            }
                        }
                        Err(_) => {}
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
struct SellOrderRequest {
    node_id: String,
    price: f64,
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
    use self::schema::open_em::nodes::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;
    use self::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong";

    let session_cookie = cookie_jar.get("session_id");

    let mut has_cookie = false;
    let mut session_id_str: String = "".to_string();
    match session_cookie {
        None => message = "Session ID not found",
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
        let user_res = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection);

        match user_res {
            Ok(user_vec) => {
                message = "No matching user";
                if user_vec.len() > 0 {
                    message = "No matching node";
                    let node_res = nodes
                        .filter(node_owner.eq(user_vec[0].user_id))
                        .select(Node::as_select())
                        .load::<Node>(connection);
                    match node_res {
                        Ok(node_vec) => {
                            if node_vec.len() > 0 {
                                let new_sell_order = NewSellOrder {
                                    seller_id: user_vec[0].user_id,
                                    offered_units: sell_order_request.units,
                                    price: sell_order_request.price,
                                    producer_id: node_vec[0].node_id,
                                };
                                message = "Failed to add new sell order";
                                match diesel::insert_into(sell_orders)
                                    .values(new_sell_order)
                                    .returning(SellOrder::as_returning())
                                    .get_result(connection)
                                {
                                    Ok(order) => {
                                        message = "Sell order created successfully";
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
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
        diesel::update(users)
            .filter(session_id.eq(session_id_str))
            .set(active.eq(false))
            .execute(connection)
            .expect("Error making session id");

        message = "Account successfully deleted";
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

    let mut message = "Something went wrong";
    let mut data = Price { price: 0f64 };

    let transactions_res = transactions
        .filter(transaction_active.eq(true))
        .order_by(created_at.desc())
        .select(Transaction::as_select())
        .load::<Transaction>(connection);

    match transactions_res {
        Ok(transactions_vec) => {
            if transactions_vec.len() > 0 {
                message = "Successfully retrieved price";
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

    let mut data = UserDetails {
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

        data = UserDetails {
            email: user_email,
            credit: user_ret[0].credit,
            first_name: user_first_name,
            last_name: user_last_name,
        };

        message = "User details successfully retrieved"
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
    data = "<node_details_req>"
)]
async fn node_details(node_details_req: Json<NodeDetailsReq>, cookie_jar: &CookieJar<'_>) -> Value {
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

    let mut data = NodeDetails {
        node_id: "".to_string(),
        name: "".to_string(),
        location_x: 0.0,
        location_y: 0.0,
        units_to_produce: 0.0,
        units_to_consume: 0.0,
    };

    if has_cookie {
        let user_vec = users
            .filter(session_id.eq(session_id_str))
            .select(User::as_select())
            .load::<User>(connection)
            .expect("User does not exist");

        let node_vec = nodes
            .filter(node_id.eq(Uuid::parse_str(&*node_details_req.node_id).unwrap()))
            .filter(node_owner.eq(user_vec[0].user_id))
            .select(Node::as_select())
            .load::<Node>(connection)
            .expect("Couldn't find node");

        data.node_id = String::from(node_vec[0].node_id);
        data.name = node_vec[0].name.clone();
        data.location_x = node_vec[0].location_x;
        data.location_y = node_vec[0].location_y;
        data.units_to_produce = node_vec[0].units_generated;
        data.units_to_consume = node_vec[0].units_consumed;

        message = "Node details retrieved succesfully"
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

    let mut ret_session_id = "".to_string();

    let user_result = users
        .filter(email.eq(credentials.email))
        .select(User::as_select())
        .load::<User>(connection);

    match user_result {
        Ok(user) => {
            message = "User does not exist";
            if user.len() > 0 {
                message = "Invalid password";
                let verify = bcrypt::verify(credentials.password, &*user[0].pass_hash);
                if verify {
                    let h = bcrypt::hash(
                        user[0].user_id.to_string() + &*chrono::Utc::now().to_string(),
                    )
                    .unwrap();
                    let h2 = h.clone();
                    let h3 = h.clone();
                    match diesel::update(users)
                        .filter(email.eq(credentials.email))
                        .set(session_id.eq(h2))
                        .execute(connection)
                    {
                        Ok(_) => {
                            message = "User logged in";
                            ret_session_id = h3;
                            jar.add(Cookie::build(("session_id", h)).path("/"));
                        }
                        Err(_) => message = "Failed to update session id",
                    };
                }
            }
        }
        Err(_) => {}
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

    let mut message = "Something went wrong";

    let connection = &mut establish_connection();

    let binding = bcrypt::hash(new_user.password.clone()).unwrap();

    let new_user_insert = NewUserModel {
        email: new_user.email.clone(),
        pass_hash: binding,
    };

    let mut ret_session_id = "".to_string();

    message = "Failed to create new user";
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
            ],
        )
        .configure(rocket::Config::figment().merge(("port", 8001)))
        .attach(CORS)
}
