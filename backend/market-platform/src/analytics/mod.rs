use crate::market_interaction::{Price, TimeFrame};
use crate::user_management::Claims;
use crate::{establish_connection, schema};
use chrono::{DateTime, Utc};
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
        }
        Err(_) => {}
    }
    json!({"status": "ok",
        "message": "User's buying stats successfully retrieved",
        "data": data
    })
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
        }
        Err(_) => {}
    }
    json!({"status": "ok",
        "message": "User's selling stats successfully retrieved",
        "data": data}
    )
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserHistoryRequest {
    time_frame: TimeFrame,
}

#[post(
    "/buy_history_stat",
    format = "application/json",
    data = "<buy_history_request>"
)]
pub fn buy_history_stat(buy_history_request: Json<UserHistoryRequest>, claims: Claims) -> Value {
    use schema::open_em::buy_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = vec![];

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string(), "data": data});
    }
    let claim_user_id = user_id_parse.unwrap();

    let (timestamp_str, time_bucket) = match buy_history_request.time_frame {
        TimeFrame::Day1 => (
            "NOW() - INTERVAL '1 day'",
            "time_bucket('5 minutes', transactions.created_at)".to_string(),
        ),
        TimeFrame::Week1 => (
            "NOW() - INTERVAL '1 week'",
            "time_bucket('1 hour', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month1 => (
            "NOW() - INTERVAL '1 month'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month3 => (
            "NOW() - INTERVAL '3 months'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month6 => (
            "NOW() - INTERVAL '6 months'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Year1 => (
            "NOW() - INTERVAL '1 year'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
    };
    let select_str = time_bucket.clone() + &*", AVG(transacted_price)".to_string();

    let connection = &mut establish_connection();

    match transactions
        .inner_join(
            buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                .eq(schema::open_em::transactions::dsl::buy_order_id)),
        )
        .filter(buyer_id.eq(claim_user_id))
        .filter(
            schema::open_em::transactions::created_at.gt(diesel::dsl::sql::<
                diesel::sql_types::Timestamptz,
            >(timestamp_str)),
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
            for result in result_vec {
                data.push(Price {
                    timestamp: result.0.to_string(),
                    price: result.1,
                })
            }
            json!({"status": "ok",
                "message": "Successfully retrieved user buy history".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "Something went wrong".to_string(),
            "data": data
        }),
    }
}

#[post(
    "/sell_history_stat",
    format = "application/json",
    data = "<sell_history_request>"
)]
pub fn sell_history_stat(sell_history_request: Json<UserHistoryRequest>, claims: Claims) -> Value {
    use schema::open_em::sell_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = vec![];

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string(), "data": data});
    }
    let claim_user_id = user_id_parse.unwrap();

    let (timestamp_str, time_bucket) = match sell_history_request.time_frame {
        TimeFrame::Day1 => (
            "NOW() - INTERVAL '1 day'",
            "time_bucket('5 minutes', transactions.created_at)".to_string(),
        ),
        TimeFrame::Week1 => (
            "NOW() - INTERVAL '1 week'",
            "time_bucket('1 hour', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month1 => (
            "NOW() - INTERVAL '1 month'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month3 => (
            "NOW() - INTERVAL '3 months'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Month6 => (
            "NOW() - INTERVAL '6 months'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
        TimeFrame::Year1 => (
            "NOW() - INTERVAL '1 year'",
            "time_bucket('1 day', transactions.created_at)".to_string(),
        ),
    };
    let select_str = time_bucket.clone() + &*", AVG(transacted_price)".to_string();

    let connection = &mut establish_connection();

    match transactions
        .inner_join(
            sell_orders.on(schema::open_em::sell_orders::dsl::sell_order_id
                .eq(schema::open_em::transactions::dsl::sell_order_id)),
        )
        .filter(seller_id.eq(claim_user_id))
        .filter(
            schema::open_em::transactions::created_at.gt(diesel::dsl::sql::<
                diesel::sql_types::Timestamptz,
            >(timestamp_str)),
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
            for result in result_vec {
                data.push(Price {
                    timestamp: result.0.to_string(),
                    price: result.1,
                })
            }
            json!({"status": "ok",
                "message": "Successfully retrieved user sell history".to_string(),
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
pub struct BoughtVsSold {
    units_bought: f64,
    units_sold: f64,
}

#[post("/bought_vs_sold_stat")]
pub fn bought_vs_sold_stat(claims: Claims) -> Value {
    use schema::open_em::buy_orders::dsl::*;
    use schema::open_em::sell_orders::dsl::*;
    use schema::open_em::transactions::dsl::*;

    let mut data = BoughtVsSold {
        units_bought: 0.0,
        units_sold: 0.0,
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
        .select(diesel::dsl::sql::<diesel::sql_types::Double>(
            "SUM(transacted_units)",
        ))
        .first::<f64>(connection)
    {
        Ok(result) => data.units_bought = result,
        Err(_) => {}
    }

    match transactions
        .inner_join(
            sell_orders.on(schema::open_em::sell_orders::dsl::sell_order_id
                .eq(schema::open_em::transactions::dsl::sell_order_id)),
        )
        .filter(seller_id.eq(claim_user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::Double>(
            "SUM(transacted_units)",
        ))
        .first::<f64>(connection)
    {
        Ok(result) => data.units_sold = result,
        Err(_) => {}
    }
    json!({"status": "ok",
        "message": "Successfully retrieved user bought and sold units".to_string(),
        "data": data
    })
}
