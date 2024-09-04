use crate::models::{
    BuyOrder, NewBuyOrder, NewSellOrder, NewTransaction, Node, SellOrder, Transaction,
};
use crate::user_management::Claims;
use crate::{
    establish_connection, schema, IMPEDANCE_RATE, SUPPLY_DEMAND_RATE, TRANSACTION_LIFETIME,
    UNIT_PRICE_RATE,
};
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use rocket::serde::{
    json::{serde_json::json, Json, Value},
    Deserialize, Serialize,
};
use std::env;
use uuid::Uuid;

#[post("/average_buy_stat")]
pub fn average_buy_stat(claims: Claims) -> Value {

    json!({"status": "ok"})
}

#[post("/average_sell_stat")]
pub fn average_sell_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}

#[post("/min_buy_stat")]
pub fn min_buy_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}

#[post("/max_sell_stat")]
pub fn max_sell_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}

#[post("/buy_history_stat")]
pub fn buy_history_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}

#[post("/sell_history_stat")]
pub fn sell_history_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}

#[post("/bought_vs_sold_stat")]
pub fn bought_vs_sold_stat(claims: Claims) -> Value {
    json!({"status": "ok"})
}
