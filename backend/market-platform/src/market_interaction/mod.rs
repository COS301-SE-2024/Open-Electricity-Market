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

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct GridStats {
    total_impedance: f64,
    total_generation: f64,
    consumer_count: i64,
    producer_count: i64,
    user_count: i64,
}

fn buy_fee_calc(units: f64, price: f64) -> f64 {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut demand = 0f64;

    match buy_orders
        .filter(sought_units.gt(filled_units))
        .filter(schema::open_em::buy_orders::dsl::active.eq(true))
        .execute(connection)
    {
        Ok(num_buys) => demand = num_buys as f64,
        Err(_) => {}
    }

    let mut supply = 0f64;
    match sell_orders
        .filter(offered_units.gt(claimed_units))
        .filter(schema::open_em::sell_orders::dsl::active.eq(true))
        .execute(connection)
    {
        Ok(num_sells) => supply = num_sells as f64,
        Err(_) => {}
    }

    let mut impedance = 1f64;
    match env::var("GRID_URL") {
        Ok(grid_url) => {
            let client = reqwest::blocking::Client::new();
            match client.post(grid_url + "/stats").send() {
                Ok(response) => match response.json::<GridStats>() {
                    Ok(grid_stats) => impedance = grid_stats.total_impedance,
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    let mut demand_supply_diff = 1f64;
    if supply < demand {
        demand_supply_diff = demand - supply
    }

    units * price * UNIT_PRICE_RATE
        + (f64::log10(demand_supply_diff) * SUPPLY_DEMAND_RATE)
        + (f64::log10(impedance) * IMPEDANCE_RATE)
}

fn sell_fee_calc(units: f64, price: f64) -> f64 {
    use self::schema::open_em::buy_orders::dsl::*;
    use self::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let mut demand = 0f64;

    match buy_orders
        .filter(sought_units.gt(filled_units))
        .filter(schema::open_em::buy_orders::dsl::active.eq(true))
        .execute(connection)
    {
        Ok(num_buys) => demand = num_buys as f64,
        Err(_) => {}
    }

    let mut supply = 0f64;
    match sell_orders
        .filter(offered_units.gt(claimed_units))
        .filter(schema::open_em::sell_orders::dsl::active.eq(true))
        .execute(connection)
    {
        Ok(num_sells) => supply = num_sells as f64,
        Err(_) => {}
    }

    let mut impedance = 1f64;
    match env::var("GRID_URL") {
        Ok(grid_url) => {
            let client = reqwest::blocking::Client::new();
            match client.post(grid_url + "/stats").send() {
                Ok(response) => match response.json::<GridStats>() {
                    Ok(grid_stats) => impedance = grid_stats.total_impedance,
                    Err(_) => {}
                },
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    let mut supply_demand_diff = 1f64;
    if demand < supply {
        supply_demand_diff = supply - demand
    }

    (units * price * UNIT_PRICE_RATE)
        + (f64::log10(supply_demand_diff) * SUPPLY_DEMAND_RATE)
        + (f64::log10(impedance) * IMPEDANCE_RATE)
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Price {
    pub price: f64,
    pub timestamp: String,
}

#[post("/price_view")]
pub fn price_view() -> Value {
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = Price {
        price: 0f64,
        timestamp: Utc::now().to_string(),
    };

    match transactions
        .filter(
            created_at.eq(diesel::dsl::sql::<diesel::sql_types::Timestamptz>(
                "(SELECT MAX(created_at) FROM transactions)",
            )),
        )
        .select(Transaction::as_select())
        .first(connection)
    {
        Ok(transaction) => {
            data = Price {
                price: transaction.transacted_price,
                timestamp: transaction.created_at.to_string(),
            };
            json!({"status": "ok", "message": "Successfully retrieved price".to_string(), "data": data})
        }
        Err(_) => {
            json!({"status": "error", "message": "Something went wrong".to_string(), "data": data})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum TimeFrame {
    Day1,
    Week1,
    Month1,
    Month3,
    Month6,
    Year1,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PriceHistoryRequest {
    time_frame: TimeFrame,
}

#[post(
    "/price_history",
    format = "application/json",
    data = "<price_history_request>"
)]
pub fn price_history(price_history_request: Json<PriceHistoryRequest>) -> Value {
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    let (timestamp_str, time_bucket) = match price_history_request.time_frame {
        TimeFrame::Day1 => (
            "NOW() - INTERVAL '1 day'",
            "time_bucket('5 minutes', created_at)".to_string(),
        ),
        TimeFrame::Week1 => (
            "NOW() - INTERVAL '1 week'",
            "time_bucket('1 hour', created_at)".to_string(),
        ),
        TimeFrame::Month1 => (
            "NOW() - INTERVAL '1 month'",
            "time_bucket('1 day', created_at)".to_string(),
        ),
        TimeFrame::Month3 => (
            "NOW() - INTERVAL '3 months'",
            "time_bucket('1 day', created_at)".to_string(),
        ),
        TimeFrame::Month6 => (
            "NOW() - INTERVAL '6 months'",
            "time_bucket('1 day', created_at)".to_string(),
        ),
        TimeFrame::Year1 => (
            "NOW() - INTERVAL '1 year'",
            "time_bucket('1 day', created_at)".to_string(),
        ),
    };

    let select_str = time_bucket.clone() + &*", AVG(transacted_price)".to_string();

    match transactions
        .filter(
            created_at.gt(diesel::dsl::sql::<diesel::sql_types::Timestamptz>(
                timestamp_str,
            )),
        )
        .group_by(diesel::dsl::sql::<diesel::sql_types::Timestamptz>(
            &*(time_bucket.clone()),
        ))
        .order_by(diesel::dsl::sql::<diesel::sql_types::Timestamptz>(
            &*(time_bucket),
        ))
        .select(diesel::dsl::sql::<(
            diesel::sql_types::Timestamptz,
            diesel::sql_types::Double,
        )>(&*select_str))
        .load::<(DateTime<Utc>, f64)>(connection)
    {
        Ok(result_vec) => {
            // if result_vec.len() == 0 {
            //     return json!({"status": "ok",
            //         "message": "Successfully retrieved price history".to_string(),
            //         "data": data
            //     })
            // }
            for result in result_vec {
                data.push(Price {
                    price: result.1,
                    timestamp: result.0.to_string(),
                })
            }
            json!({"status": "ok",
                "message": "Successfully retrieved price history".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "Something went wrong".to_string(),
            "data": data
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OrderRequest {
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
pub fn buy_order(buy_order_request: Json<OrderRequest>, claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*buy_order_request.node_id);
    if node_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid Node ID".to_string()});
    }
    let request_node_id = node_id_parse.unwrap();

    let connection = &mut establish_connection();

    match nodes
        .filter(node_id.eq(request_node_id))
        .filter(node_owner.eq(claim_user_id))
        .filter(node_active.eq(true))
        .select(Node::as_select())
        .first(connection)
    {
        Ok(node) => {
            if buy_order_request.max_price > 0f64
                && buy_order_request.min_price > 0f64
                && buy_order_request.units > 0f64
            {
                let new_buy_order = NewBuyOrder {
                    buyer_id: claim_user_id,
                    consumer_id: node.node_id,
                    sought_units: buy_order_request.units,
                    max_price: buy_order_request.max_price,
                    min_price: buy_order_request.min_price,
                };
                match diesel::insert_into(buy_orders)
                    .values(new_buy_order)
                    .returning(BuyOrder::as_returning())
                    .get_result(connection)
                {
                    Ok(mut order) => {
                        match sell_orders
                            .filter(offered_units.gt(claimed_units))
                            .filter(schema::open_em::sell_orders::max_price.le(order.max_price))
                            .filter(seller_id.ne(order.buyer_id))
                            .filter(producer_id.ne(order.consumer_id))
                            .filter(schema::open_em::sell_orders::dsl::active.eq(true))
                            .order_by(schema::open_em::sell_orders::created_at.asc())
                            .select(SellOrder::as_select())
                            .load::<SellOrder>(connection)
                        {
                            Ok(sell_order_vec) => {
                                let mut order_match = false;
                                for s_order in sell_order_vec {
                                    let transaction_units: f64;
                                    if s_order.offered_units - s_order.claimed_units
                                        > order.sought_units - order.filled_units
                                    {
                                        transaction_units = order.sought_units - order.filled_units;
                                    } else {
                                        transaction_units =
                                            s_order.offered_units - s_order.claimed_units;
                                    }
                                    let transaction_price = s_order.max_price; // Will be based on the direction the market needs to move for grid stability
                                    let fee = buy_fee_calc(transaction_units, transaction_price);
                                    let new_transaction = NewTransaction {
                                        sell_order_id: s_order.sell_order_id,
                                        buy_order_id: order.buy_order_id,
                                        transacted_units: transaction_units,
                                        transacted_price: transaction_price,
                                        transaction_fee: fee,
                                    };
                                    match diesel::insert_into(transactions)
                                        .values(new_transaction)
                                        .returning(Transaction::as_returning())
                                        .get_result(connection)
                                    {
                                        Ok(transaction) => {
                                            order_match = true;
                                            order.filled_units += transaction.transacted_units
                                        }
                                        Err(_) => {}
                                    }
                                    if order.filled_units == order.sought_units {
                                        break;
                                    }
                                }
                                if order_match {
                                    return json!({"status": "ok",
                                        "message": "Buy order created successfully. Order matched".to_string()
                                    });
                                }
                                return json!({"status": "ok",
                                    "message": "Buy order created successfully. Pending match".to_string()
                                });
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {
                        return json!({"status": "error", "message": "Failed to add new buy order".to_string()})
                    }
                }
            }
            json!({"status": "error", "message": "Invalid price or units".to_string()})
        }
        Err(_) => json!({"status": "error", "message": "No matching node".to_string()}),
    }
}

#[post(
    "/sell_order",
    format = "application/json",
    data = "<sell_order_request>"
)]
pub fn sell_order(sell_order_request: Json<OrderRequest>, claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*sell_order_request.node_id);
    if node_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid Node ID".to_string()});
    }
    let request_node_id = node_id_parse.unwrap();

    match nodes
        .filter(node_id.eq(request_node_id))
        .filter(node_owner.eq(claim_user_id))
        .filter(node_active.eq(true))
        .select(Node::as_select())
        .first(connection)
    {
        Ok(node) => {
            if sell_order_request.max_price > 0f64
                && sell_order_request.min_price > 0f64
                && sell_order_request.units > 0f64
            {
                let new_sell_order = NewSellOrder {
                    seller_id: claim_user_id,
                    offered_units: sell_order_request.units,
                    max_price: sell_order_request.max_price,
                    min_price: sell_order_request.min_price,
                    producer_id: node.node_id,
                };
                match diesel::insert_into(sell_orders)
                    .values(new_sell_order)
                    .returning(SellOrder::as_returning())
                    .get_result(connection)
                {
                    Ok(mut order) => {
                        match buy_orders
                            .filter(sought_units.gt(filled_units))
                            .filter(schema::open_em::buy_orders::min_price.ge(order.min_price))
                            .filter(buyer_id.ne(order.seller_id))
                            .filter(consumer_id.ne(order.producer_id))
                            .filter(schema::open_em::buy_orders::dsl::active.eq(true))
                            .order_by(schema::open_em::buy_orders::created_at.asc())
                            .select(BuyOrder::as_select())
                            .load::<BuyOrder>(connection)
                        {
                            Ok(buy_order_vec) => {
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
                                    let fee = sell_fee_calc(transaction_units, transaction_price);
                                    let new_transaction = NewTransaction {
                                        buy_order_id: b_order.buy_order_id,
                                        sell_order_id: order.sell_order_id,
                                        transacted_units: transaction_units,
                                        transacted_price: transaction_price,
                                        transaction_fee: fee,
                                    };
                                    match diesel::insert_into(transactions)
                                        .values(new_transaction)
                                        .returning(Transaction::as_returning())
                                        .get_result(connection)
                                    {
                                        Ok(transaction) => {
                                            order_match = true;
                                            order.claimed_units += transaction.transacted_units;
                                        }
                                        Err(_) => {}
                                    }
                                    if order.claimed_units == order.offered_units {
                                        break;
                                    }
                                }
                                if order_match {
                                    return json!({"status": "ok",
                                    "message": "Sell order created successfully. Order matched".to_string()});
                                }
                                return json!({"status": "ok",
                                    "message": "Sell order created successfully. Pending match".to_string()});
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {
                        return json!({"status": "error",
                        "message": "Failed to add new sell order".to_string()})
                    }
                }
            }
            json!({"status": "error", "message": "Invalid price or units".to_string()})
        }
        Err(_) => json!({"status": "error", "message": "No matching node".to_string()}),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OpenSell {
    order_id: String,
    offered_units: f64,
    claimed_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_sells")]
pub fn list_open_sells(claims: Claims) -> Value {
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    match sell_orders
        .filter(seller_id.eq(claim_user_id))
        .filter(offered_units.gt(claimed_units))
        .filter(active.eq(true))
        .select(SellOrder::as_select())
        .load::<SellOrder>(connection)
    {
        Ok(order_vec) => {
            if order_vec.len() > 0 {
                for order in order_vec {
                    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                    let mut transaction_price = 0f64;
                    match transactions
                        .filter(
                            schema::open_em::transactions::sell_order_id.eq(order.sell_order_id),
                        )
                        .filter(schema::open_em::transactions::created_at.gt(timestamp))
                        .order_by(schema::open_em::transactions::created_at.desc())
                        .select(Transaction::as_select())
                        .load::<Transaction>(connection)
                    {
                        Ok(transaction_vec) => {
                            if transaction_vec.len() > 0 {
                                transaction_price = transaction_vec[0].transacted_price
                            }
                        }
                        Err(_) => {}
                    }
                    data.push(OpenSell {
                        order_id: String::from(order.sell_order_id),
                        offered_units: order.offered_units,
                        claimed_units: order.claimed_units,
                        max_price: order.max_price,
                        min_price: order.min_price,
                        last_transacted_price: transaction_price,
                    })
                }
                return json!({"status": "ok",
                    "message": "Successfully retrieved open sell orders".to_string(),
                    "data": data
                });
            }
            json!({"status": "ok",
                "message": "No open sell orders".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "Something went wrong.".to_string(),
            "data": data
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct OpenBuy {
    order_id: String,
    sought_units: f64,
    filled_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_buys")]
pub fn list_open_buys(claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    match buy_orders
        .filter(buyer_id.eq(claim_user_id))
        .filter(sought_units.gt(filled_units))
        .filter(active.eq(true))
        .select(BuyOrder::as_select())
        .load::<BuyOrder>(connection)
    {
        Ok(order_vec) => {
            if order_vec.len() > 0 {
                for order in order_vec {
                    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                    let mut transaction_price = 0f64;
                    match transactions
                        .filter(schema::open_em::transactions::buy_order_id.eq(order.buy_order_id))
                        .filter(schema::open_em::transactions::created_at.gt(timestamp))
                        .order_by(schema::open_em::transactions::created_at.desc())
                        .select(Transaction::as_select())
                        .load::<Transaction>(connection)
                    {
                        Ok(transaction_vec) => {
                            if transaction_vec.len() > 0 {
                                transaction_price = transaction_vec[0].transacted_price
                            }
                        }
                        Err(_) => {}
                    }
                    data.push(OpenBuy {
                        order_id: String::from(order.buy_order_id),
                        sought_units: order.sought_units,
                        filled_units: order.filled_units,
                        max_price: order.max_price,
                        min_price: order.min_price,
                        last_transacted_price: transaction_price,
                    })
                }
                return json!({"status": "ok",
                    "message": "Successfully retrieved open buy orders".to_string(),
                    "data": data
                });
            }
            json!({"status": "ok",
                "message": "No open buy orders".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "Something went wrong.".to_string(),
            "data": data
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ListAllRequest {
    limit: u64,
}

#[post(
    "/all_open_buy",
    format = "application/json",
    data = "<all_open_buy_request>"
)]
pub fn all_open_buy(all_open_buy_request: Json<ListAllRequest>) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    match buy_orders
        .filter(sought_units.gt(filled_units))
        .filter(active.eq(true))
        .order_by(schema::open_em::buy_orders::buy_order_id)
        .select(BuyOrder::as_select())
        .limit(all_open_buy_request.limit as i64)
        .load::<BuyOrder>(connection)
    {
        Ok(order_vec) => {
            if order_vec.len() > 0 {
                for order in order_vec {
                    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                    let mut transaction_price = 0f64;
                    match transactions
                        .filter(schema::open_em::transactions::buy_order_id.eq(order.buy_order_id))
                        .filter(schema::open_em::transactions::created_at.gt(timestamp))
                        .order_by(schema::open_em::transactions::created_at.desc())
                        .select(Transaction::as_select())
                        .load::<Transaction>(connection)
                    {
                        Ok(transaction_vec) => {
                            if transaction_vec.len() > 0 {
                                transaction_price = transaction_vec[0].transacted_price
                            }
                        }
                        Err(_) => {}
                    }
                    data.push(OpenBuy {
                        order_id: String::from(order.buy_order_id),
                        sought_units: order.sought_units,
                        filled_units: order.filled_units,
                        max_price: order.max_price,
                        min_price: order.min_price,
                        last_transacted_price: transaction_price,
                    })
                }
                return json!({"status": "ok",
                    "message": "Successfully retrieved open buy orders".to_string(),
                    "data": data
                });
            }
            json!({"status": "ok",
                "message": "No open buy orders".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message":  "Something went wrong.".to_string(),
            "data": data
        }),
    }
}

#[post(
    "/all_open_sell",
    format = "application/json",
    data = "<all_open_sell_request>"
)]
pub fn all_open_sell(all_open_sell_request: Json<ListAllRequest>) -> Value {
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut data = vec![];

    match sell_orders
        .filter(offered_units.gt(claimed_units))
        .filter(active.eq(true))
        .order_by(schema::open_em::sell_orders::sell_order_id.asc())
        .select(SellOrder::as_select())
        .limit(all_open_sell_request.limit as i64)
        .load::<SellOrder>(connection)
    {
        Ok(order_vec) => {
            if order_vec.len() > 0 {
                for order in order_vec {
                    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                    let mut transaction_price = 0f64;
                    match transactions
                        .filter(
                            schema::open_em::transactions::sell_order_id.eq(order.sell_order_id),
                        )
                        .filter(schema::open_em::transactions::created_at.gt(timestamp))
                        .order_by(schema::open_em::transactions::created_at.desc())
                        .select(Transaction::as_select())
                        .load::<Transaction>(connection)
                    {
                        Ok(transaction_vec) => {
                            if transaction_vec.len() > 0 {
                                transaction_price = transaction_vec[0].transacted_price
                            }
                        }
                        Err(_) => {}
                    }
                    data.push(OpenSell {
                        order_id: String::from(order.sell_order_id),
                        offered_units: order.offered_units,
                        claimed_units: order.claimed_units,
                        max_price: order.max_price,
                        min_price: order.min_price,
                        last_transacted_price: transaction_price,
                    })
                }
                return json!({"status": "ok",
                    "message": "Successfully retrieved open sell orders".to_string(),
                    "data": data
                });
            }
            json!({"status": "ok",
                "message": "No open sell orders".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "Something went wrong.".to_string(),
            "data": data
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FeeEstimationRequest {
    price: f64,
    units: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FeeEstimation {
    fee: f64,
}

#[post(
    "/estimate_buy_fee",
    format = "application/json",
    data = "<fee_estimation_request>"
)]
pub fn estimate_buy_fee(fee_estimation_request: Json<FeeEstimationRequest>) -> Value {
    let message = "Buy fee estimation".to_string();

    let temp = buy_fee_calc(fee_estimation_request.units, fee_estimation_request.price);

    let data = FeeEstimation { fee: temp };

    json!({"status": "ok", "message": message, "data": data})
}

#[post(
    "/estimate_sell_fee",
    format = "application/json",
    data = "<fee_estimation_request>"
)]
pub fn estimate_sell_fee(fee_estimation_request: Json<FeeEstimationRequest>) -> Value {
    let message = "Sell fee estimation".to_string();

    let temp = sell_fee_calc(fee_estimation_request.units, fee_estimation_request.price);

    let data = FeeEstimation { fee: temp };
    json!({"status": "ok", "message": message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CancelOrderRequest {
    order_id: String,
}

#[post(
    "/cancel_buy_order",
    format = "application/json",
    data = "<cancel_buy_request>"
)]
pub fn cancel_buy_order(cancel_buy_request: Json<CancelOrderRequest>, claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let order_id_parse = Uuid::parse_str(&*cancel_buy_request.order_id);
    if order_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid Order ID".to_string()});
    }
    let request_order_id = order_id_parse.unwrap();

    let connection = &mut establish_connection();

    match diesel::update(buy_orders)
        .filter(buyer_id.eq(claim_user_id))
        .filter(buy_order_id.eq(request_order_id))
        .filter(active.eq(true))
        .set(active.eq(false))
        .execute(connection)
    {
        Ok(_) => {
            json!({"status": "ok", "message": "Order successfully cancelled"})
        }
        Err(_) => {
            json!({"status": "ok", "message": "Order already cancelled"})
        }
    }
}

#[post(
    "/cancel_sell_order",
    format = "application/json",
    data = "<cancel_sell_request>"
)]
pub fn cancel_sell_order(cancel_sell_request: Json<CancelOrderRequest>, claims: Claims) -> Value {
    use crate::schema::open_em::sell_orders::dsl::*;

    let connection = &mut establish_connection();

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let order_id_parse = Uuid::parse_str(&*cancel_sell_request.order_id);
    if order_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid Order ID".to_string()});
    }
    let request_order_id = order_id_parse.unwrap();

    match diesel::update(sell_orders)
        .filter(seller_id.eq(claim_user_id))
        .filter(sell_order_id.eq(request_order_id))
        .filter(active.eq(true))
        .set(active.eq(false))
        .execute(connection)
    {
        Ok(_) => {
            json!({"status": "ok", "message": "Order successfully cancelled"})
        }
        Err(_) => {
            json!({"status": "ok", "message": "Order already cancelled"})
        }
    }
}
