use crate::user_management::Claims;
use crate::{
    establish_connection, schema,
};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use rocket::serde::{
    json::{serde_json::json, Json, Value},
    Deserialize, Serialize,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AvgStatResponse {
    average_price: f64,
}

#[post("/average_buy_stat")]
pub fn average_buy_stat(claims: Claims) -> Value {
    use schema::open_em::buy_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = AvgStatResponse { average_price: 0.0 };

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let connection = &mut establish_connection();

    match transactions
        .inner_join(
            buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                .eq(schema::open_em::transactions::dsl::buy_order_id)),
        )
        .filter(buyer_id.eq(claim_user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::Double>(
            "AVG(transacted_price)",
        ))
        .first::<f64>(connection)
    {
        Ok(result) => {
            data.average_price = result;
            json!(
                {"status": "ok",
                "message": "User's average buying price successfully retrieved",
                "data": data}
            )
        }
        Err(_) => json!({"status": "err", "message": "Something went wrong", "data": data}),
    }
}

#[post("/average_sell_stat")]
pub fn average_sell_stat(claims: Claims) -> Value {
    use schema::open_em::sell_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = AvgStatResponse { average_price: 0.0 };

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string(), "data": data});
    }
    let claim_user_id = user_id_parse.unwrap();

    let connection = &mut establish_connection();

    match transactions
        .inner_join(
            sell_orders.on(schema::open_em::sell_orders::dsl::sell_order_id
                .eq(schema::open_em::transactions::dsl::sell_order_id)),
        )
        .filter(seller_id.eq(claim_user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::Double>(
            "AVG(transacted_price)",
        ))
        .first::<f64>(connection)
    {
        Ok(result) => {
            data.average_price = result;
            json!(
                {"status": "ok",
                "message": "User's average selling price successfully retrieved",
                "data": data}
            )
        }
        Err(_) => json!({"status": "err", "message": "Something went wrong", "data": data}),
    }
}

#[post("/min_buy_stat")]
pub fn min_buy_stat(claims: Claims) -> Value {
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    // let claim_user_id = user_id_parse.unwrap();
    json!({"status": "ok"})
}

#[post("/max_sell_stat")]
pub fn max_sell_stat(claims: Claims) -> Value {
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    // let claim_user_id = user_id_parse.unwrap();
    json!({"status": "ok"})
}

#[post("/buy_history_stat")]
pub fn buy_history_stat(claims: Claims) -> Value {
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    // let claim_user_id = user_id_parse.unwrap();
    json!({"status": "ok"})
}

#[post("/sell_history_stat")]
pub fn sell_history_stat(claims: Claims) -> Value {
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    // let claim_user_id = user_id_parse.unwrap();
    json!({"status": "ok"})
}

#[post("/bought_vs_sold_stat")]
pub fn bought_vs_sold_stat(claims: Claims) -> Value {
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    // let claim_user_id = user_id_parse.unwrap();
    json!({"status": "ok"})
}
