use crate::models::{
    BuyOrder, NewBuyOrder, NewSellOrder, NewTransaction, Node, SellOrder, Transaction,
};
use crate::{
    buy_fee_calc, establish_connection, schema, sell_fee_calc, verify_user, TARGET_HISTORY_POINTS,
    TRANSACTION_LIFETIME,
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rocket::http::CookieJar;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Price {
    price: f64,
    timestamp: String,
}

#[post("/price_view")]
pub fn price_view() -> Value {
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong".to_string();
    let mut data = Price {
        price: 0f64,
        timestamp: Utc::now().to_string(),
    };

    let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

    match transactions
        .filter(created_at.gt(timestamp))
        .order_by(created_at.desc())
        .select(Transaction::as_select())
        .load::<Transaction>(connection)
    {
        Ok(transactions_vec) => {
            if transactions_vec.len() > 0 {
                message = "Successfully retrieved price".to_string();
                data = Price {
                    price: transactions_vec[0].transacted_price,
                    timestamp: transactions_vec[0].created_at.to_string(),
                }
            }
        }
        Err(_) => {}
    }

    json!({"status": "ok", "message":message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PriceHistoryRequest {
    hours: i64,
}

#[post(
    "/price_history",
    format = "application/json",
    data = "<price_history_request>"
)]
pub fn price_history(price_history_request: Json<PriceHistoryRequest>) -> Value {
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let mut message = "Something went wrong".to_string();
    let mut data = vec![];

    let timestamp = Utc::now() - Duration::hours(price_history_request.hours);

    match transactions
        .filter(created_at.gt(timestamp))
        .order_by(created_at.asc())
        .select(Transaction::as_select())
        .load::<Transaction>(connection)
    {
        Ok(transactions_vec) => {
            message = "Successfully retrieved price".to_string();
            if transactions_vec.len() as i64 <= TARGET_HISTORY_POINTS {
                for transaction in transactions_vec {
                    data.push(Price {
                        price: transaction.transacted_price,
                        timestamp: transaction.created_at.to_string(),
                    })
                }
            } else {
                let interval = transactions_vec.len() / 100;
                let mut count = 0;
                let mut interval_count = 0usize;
                while count < transactions_vec.len() {
                    if interval_count == interval {
                        data.push(Price {
                            price: transactions_vec[count].transacted_price,
                            timestamp: transactions_vec[count].created_at.to_string(),
                        });
                        interval_count = 0;
                    }
                    interval_count += 1;
                    count += 1;
                }
            }
        }
        Err(_) => {}
    }

    json!({"status": "ok", "message":message, "data": data})
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
pub fn buy_order(buy_order_request: Json<BuyOrderRequest>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

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
                        message = "Invalid price of units".to_string();
                        if buy_order_request.max_price > 0f64
                            && buy_order_request.min_price > 0f64
                            && buy_order_request.units > 0f64
                        {
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
                                            schema::open_em::sell_orders::max_price
                                                .le(order.max_price),
                                        )
                                        .filter(seller_id.ne(order.buyer_id))
                                        .filter(producer_id.ne(order.consumer_id))
                                        .order_by(schema::open_em::sell_orders::created_at.asc())
                                        .select(SellOrder::as_select())
                                        .load::<SellOrder>(connection)
                                    {
                                        Ok(sell_order_vec) => {
                                            message =
                                                "Buy order created successfully. Pending match"
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
                                                    transaction_units = s_order.offered_units
                                                        - s_order.claimed_units;
                                                }
                                                let transaction_price = s_order.max_price; // Will be based on the direction the market needs to move for grid stability
                                                let fee = buy_fee_calc(
                                                    transaction_units,
                                                    transaction_price,
                                                );
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
                                                        order.filled_units +=
                                                            transaction.transacted_units
                                                    }
                                                    Err(_) => {
                                                        message =
                                                            "Transaction(s) failed".to_string()
                                                        //message = error.to_string().clone();
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
pub fn sell_order(sell_order_request: Json<SellOrderRequest>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

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
                        message = "Invalid price or units".to_string();
                        if sell_order_request.max_price > 0f64
                            && sell_order_request.min_price > 0f64
                            && sell_order_request.units > 0f64
                        {
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
                                            schema::open_em::buy_orders::min_price
                                                .ge(order.min_price),
                                        )
                                        .filter(buyer_id.ne(order.seller_id))
                                        .filter(consumer_id.ne(order.producer_id))
                                        .order_by(schema::open_em::buy_orders::created_at.asc())
                                        .select(BuyOrder::as_select())
                                        .load::<BuyOrder>(connection)
                                    {
                                        Ok(buy_order_vec) => {
                                            message =
                                                "Sell order created successfully. Pending match"
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
                                                let fee = sell_fee_calc(
                                                    transaction_units,
                                                    transaction_price,
                                                );
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
                                                        order.claimed_units +=
                                                            transaction.transacted_units;
                                                    }
                                                    Err(_) => {
                                                        message =
                                                            "Transaction(s) failed".to_string()
                                                        //message = error.to_string().clone();
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
struct OpenSell {
    order_id: i64,
    offered_units: f64,
    claimed_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_sells")]
pub fn list_open_sells(cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

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
                        let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                        let mut transaction_price = 0f64;
                        match transactions
                            .filter(
                                schema::open_em::transactions::sell_order_id
                                    .eq(order.sell_order_id),
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
                            order_id: order.sell_order_id,
                            offered_units: order.offered_units,
                            claimed_units: order.claimed_units,
                            max_price: order.max_price,
                            min_price: order.min_price,
                            last_transacted_price: transaction_price,
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
struct OpenBuy {
    order_id: i64,
    sought_units: f64,
    filled_units: f64,
    max_price: f64,
    min_price: f64,
    last_transacted_price: f64,
}

#[post("/list_open_buys")]
pub fn list_open_buys(cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

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
                        let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);
                        let mut transaction_price = 0f64;
                        match transactions
                            .filter(
                                schema::open_em::transactions::buy_order_id.eq(order.buy_order_id),
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
                        data.push(OpenBuy {
                            order_id: order.buy_order_id,
                            sought_units: order.sought_units,
                            filled_units: order.filled_units,
                            max_price: order.max_price,
                            min_price: order.min_price,
                            last_transacted_price: transaction_price,
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
struct FeeEstimationRequest {
    price: f64,
    units: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct FeeEstimation {
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
