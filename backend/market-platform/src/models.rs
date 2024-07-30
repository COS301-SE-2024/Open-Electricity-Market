use diesel::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub pass_hash: String,
    pub credit: f64,
    pub created_at: DateTime<Utc>,
    pub session_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::users)]
pub struct NewUserModel<'a> {
    pub email: &'a str,
    pub pass_hash: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::profiles)]
pub struct NewProfileModel<'a> {
    pub user_id: &'a Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::sell_orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SellOrder {
    pub sell_order_id: i64,
    pub seller_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub offered_units: f64,
    pub claimed_units: f64,
    pub price: f64,
    pub producer_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::sell_orders)]
pub struct NewSellOrderModel<'a> {
    pub seller_id: &'a Uuid,
    pub producer_id: &'a Uuid,
    pub offered_units: &'a f64,
    pub price: &'a f64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::buy_orders)]
pub struct NewBuyOrderModel<'a> {
    pub buyer_id: &'a Uuid,
    pub consumer_id: &'a Uuid,
    pub sought_units: &'a f64,
    pub price: &'a f64,
}
