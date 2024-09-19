use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub pass_hash: String,
    pub credit: f64,
    // pub created_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::users)]
pub struct NewUserModel {
    pub email: String,
    pub pass_hash: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::profiles)]
pub struct Profile {
    // pub profile_user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::profiles)]
pub struct NewProfileModel {
    pub profile_user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::nodes)]
pub struct Node {
    pub node_id: Uuid,
    // pub node_owner: Uuid,
    pub location_x: f64,
    pub location_y: f64,
    // pub node_active: bool,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::buy_orders)]
pub struct NewBuyOrder {
    pub buyer_id: Uuid,
    pub sought_units: f64,
    pub max_price: f64,
    pub consumer_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::buy_orders)]
pub struct BuyOrder {
    pub buy_order_id: Uuid,
    pub buyer_id: Uuid,
    pub sought_units: f64,
    pub filled_units: f64,
    pub max_price: f64,
    // pub created_at: DateTime<Utc>,
    pub consumer_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::sell_orders)]
pub struct NewSellOrder {
    pub seller_id: Uuid,
    pub offered_units: f64,
    pub min_price: f64,
    pub producer_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::sell_orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SellOrder {
    pub sell_order_id: Uuid,
    pub seller_id: Uuid,
    // pub created_at: DateTime<Utc>,
    pub offered_units: f64,
    pub claimed_units: f64,
    pub min_price: f64,
    pub producer_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::nodes)]
pub struct NewNodeModel<'a> {
    pub node_owner: Uuid,
    pub location_x: f64,
    pub location_y: f64,
    pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::transactions)]
pub struct NewTransaction {
    pub sell_order_id: Uuid,
    pub buy_order_id: Uuid,
    pub transacted_units: f64,
    pub transacted_price: f64,
    pub transaction_fee: f64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::transactions)]
pub struct Transaction {
    pub transaction_id: Uuid,
    // pub sell_order_id: Uuid,
    // pub buy_order_id: Uuid,
    pub transacted_units: f64,
    pub transacted_price: f64,
    // pub transaction_fee: f64,
    pub units_consumed: f64,
    pub units_produced: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::funds)]
pub struct NewFundModel {
    pub fund_holder: Uuid,
    pub amount: f64,
}
