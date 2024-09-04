use crate::user_management::Claims;
use crate::{establish_connection, schema};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use rocket::serde::{
    json::{serde_json::json, Json, Value},
    Deserialize, Serialize,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct StatResponse {
    min_price: f64,
    max_price: f64,
    average_price: f64,
}

#[post("/user_buy_stats")]
pub fn user_buy_stats(claims: Claims) -> Value {
    use schema::open_em::buy_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = StatResponse {
        min_price: 0.0,
        max_price: 0.0,
        average_price: 0.0,
    };

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
        .select(diesel::dsl::sql::<(
            diesel::sql_types::Double,
            diesel::sql_types::Double,
            diesel::sql_types::Double,
        )>(
            "MIN(transacted_price), MAX(transacted_price), AVG(transacted_price)",
        ))
        .first::<(f64, f64, f64)>(connection)
    {
        Ok(result) => {
            data.min_price = result.0;
            data.max_price = result.1;
            data.average_price = result.2;
            json!(
                {"status": "ok",
                "message": "User's buying stats successfully retrieved",
                "data": data}
            )
        }
        Err(_) => json!({"status": "err", "message": "Something went wrong", "data": data}),
    }
}

#[post("/user_sell_stats")]
pub fn user_sell_stats(claims: Claims) -> Value {
    use schema::open_em::sell_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = StatResponse {
        min_price: 0.0,
        max_price: 0.0,
        average_price: 0.0,
    };

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
        .select(diesel::dsl::sql::<(
            diesel::sql_types::Double,
            diesel::sql_types::Double,
            diesel::sql_types::Double,
        )>(
            "MIN(transacted_price), MAX(transacted_price), AVG(transacted_price)",
        ))
        .first::<(f64, f64, f64)>(connection)
    {
        Ok(result) => {
            data.min_price = result.0;
            data.max_price = result.1;
            data.average_price = result.2;
            json!(
                {"status": "ok",
                "message": "User's selling stats successfully retrieved",
                "data": data}
            )
        }
        Err(_) => json!({"status": "err", "message": "Something went wrong", "data": data}),
    }
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
