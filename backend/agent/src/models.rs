use diesel::prelude::*;
use uuid::Uuid;

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
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::buy_orders)]
pub struct BuyOrder {
    pub buy_order_id: Uuid,
    pub buyer_id: Uuid,
    pub sought_units: f64,
    pub filled_units: f64,
    pub max_price: f64,
    pub min_price: f64,
    // pub created_at: DateTime<Utc>,
    pub consumer_id: Uuid,
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
    pub max_price: f64,
    pub min_price: f64,
    pub producer_id: Uuid,
}
