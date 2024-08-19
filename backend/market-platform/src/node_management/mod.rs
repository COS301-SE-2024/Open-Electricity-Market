use crate::models::{BuyOrder, NewNodeModel, Node, SellOrder, Transaction};
use crate::user_management::verify_user;
use crate::{establish_connection, schema, TRANSACTION_LIFETIME};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rocket::http::CookieJar;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddNodeReq<'r> {
    name: &'r str,
    location_x: f64,
    location_y: f64,
}

#[post("/add_node", format = "application/json", data = "<add_node_req>")]
pub fn add_node(add_node_req: Json<AddNodeReq<'_>>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::nodes;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.message == "User found" {
        let new_node_insert = NewNodeModel {
            node_owner: claims.user_id,
            location_x: add_node_req.location_x,
            location_y: add_node_req.location_y,
            name: add_node_req.name,
        };

        match diesel::insert_into(nodes::table)
            .values(&new_node_insert)
            .execute(connection)
        {
            Ok(_) => message = "New Node Added".to_string(),
            Err(_) => message = "Something went wrong".to_string(),
        }
    }

    json!({"status": "ok", "message": message})
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

#[post(
    "/get_nodes",
    format = "application/json",
    data = "<get_nodes_request>"
)]
pub fn get_nodes(get_nodes_request: Json<GetNodesReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::nodes::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    let mut node_list: Vec<ShortNodeRet> = vec![];

    if claims.message == "User found" {
        match nodes
            .filter(node_owner.eq(claims.user_id))
            .filter(node_active.eq(true))
            .select(Node::as_select())
            .limit(get_nodes_request.limit)
            .load::<Node>(connection)
        {
            Ok(node_vec) => {
                for node in node_vec {
                    node_list.push(ShortNodeRet {
                        node_id: node.node_id.to_string(),
                        name: node.name,
                    })
                }
                message = "List of nodes successfully retrieved".to_string()
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message, "data": node_list})
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
    data = "<node_details_request>"
)]
pub fn node_details(
    node_details_request: Json<NodeDetailsReq>,
    cookie_jar: &CookieJar<'_>,
) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut data = NodeDetails {
        node_id: "".to_string(),
        name: "".to_string(),
        location_x: 0.0,
        location_y: 0.0,
        units_to_produce: 0.0,
        units_to_consume: 0.0,
    };

    let mut message = claims.message.clone();
    if claims.message == "User found" {
        message = "Invalid Node ID".to_string();
        match Uuid::parse_str(&*node_details_request.node_id) {
            Ok(request_node_id) => {
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_active.eq(true))
                    .select(Node::as_select())
                    .load::<Node>(connection)
                {
                    Ok(node_vec) => {
                        message = "No matching node".to_string();
                        if node_vec.len() > 0 {
                            message = "Node details retrieved succesfully".to_string();
                            data.node_id = String::from(node_vec[0].node_id);
                            data.name = node_vec[0].name.clone();
                            data.location_x = node_vec[0].location_x;
                            data.location_y = node_vec[0].location_y;

                            let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                            match transactions
                                .inner_join(sell_orders)
                                .filter(producer_id.eq(node_vec[0].node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                                    "SUM(transacted_units - units_produced)",
                                ))
                                .load::<f64>(connection)
                            // .select((Transaction::as_select(), SellOrder::as_select()))
                            // .load::<(Transaction, SellOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    if result_vec.len() > 0 {
                                        data.units_to_produce = result_vec[0];
                                    }
                                    /*message = result_vec.len().to_string();
                                    for row in result_vec {
                                        data.units_to_produce += row.0.transacted_units - row.0.units_produced;
                                    }*/
                                }
                                Err(_) => {
                                    message = "Units produced error".to_string();
                                }
                            };

                            match transactions
                                .inner_join(
                                    buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                                        .eq(schema::open_em::transactions::dsl::buy_order_id)),
                                )
                                .filter(consumer_id.eq(node_vec[0].node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                                    "SUM(transacted_units - units_consumed)",
                                ))
                                .load::<f64>(connection)
                            // .select((Transaction::as_select(), BuyOrder::as_select()))
                            // .load::<(Transaction, BuyOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    if result_vec.len() > 0 {
                                        data.units_to_consume = result_vec[0];
                                    }
                                    /*message = result_vec.len().to_string();
                                    for row in result_vec {
                                        data.units_to_consume += row.0.transacted_units - row.0.units_consumed;
                                    }*/
                                }
                                Err(_) => {}
                            };
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateUnits {
    units: f64,
    node_id: String,
}

#[post(
    "/update_consumed_units",
    format = "application/json",
    data = "<update_request>"
)]
pub fn update_consumed_units(
    mut update_request: Json<UpdateUnits>,
    cookie_jar: &CookieJar<'_>,
) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match Uuid::parse_str(&*update_request.node_id) {
            Ok(request_node_id) => {
                message = "No matching node".to_string();
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_active.eq(true))
                    .select(Node::as_select())
                    .first(connection)
                {
                    Ok(node) => {
                        message = "Invalid request units".to_string();
                        if update_request.units > 0f64 {
                            let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                            match transactions
                                .inner_join(
                                    buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                                        .eq(schema::open_em::transactions::dsl::buy_order_id)),
                                )
                                .filter(consumer_id.eq(node.node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .order_by(schema::open_em::transactions::created_at.asc())
                                .select((Transaction::as_select(), BuyOrder::as_select()))
                                .load::<(Transaction, BuyOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    message = "Insufficient available units to consume".to_string();
                                    for (transaction, _) in result_vec {
                                        if transaction.transacted_units - transaction.units_consumed
                                            == 0f64
                                        {
                                            continue;
                                        }
                                        if transaction.transacted_units - transaction.units_consumed
                                            >= update_request.units
                                        {
                                            match diesel::update(transactions)
                                                .set(units_consumed.eq(transaction.units_consumed
                                                    + update_request.units))
                                                .filter(
                                                    transaction_id.eq(transaction.transaction_id),
                                                )
                                                .execute(connection)
                                            {
                                                Ok(_) => {
                                                    message = "Units updated".to_string();
                                                    break;
                                                }
                                                Err(_) => {}
                                            }
                                        } else {
                                            match diesel::update(transactions)
                                                .set(
                                                    units_consumed.eq(transaction.transacted_units),
                                                )
                                                .filter(
                                                    transaction_id.eq(transaction.transaction_id),
                                                )
                                                .execute(connection)
                                            {
                                                Ok(_) => {
                                                    update_request.units -= transaction
                                                        .transacted_units
                                                        - transaction.units_consumed
                                                }
                                                Err(_) => {}
                                            }
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
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[post(
    "/update_produced_units",
    format = "application/json",
    data = "<update_request>"
)]
pub fn update_produced_units(
    mut update_request: Json<UpdateUnits>,
    cookie_jar: &CookieJar<'_>,
) -> Value {
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match Uuid::parse_str(&*update_request.node_id) {
            Ok(request_node_id) => {
                message = "No matching node".to_string();
                match nodes
                    .filter(node_id.eq(request_node_id))
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_active.eq(true))
                    .select(Node::as_select())
                    .first(connection)
                {
                    Ok(node) => {
                        message = "Invalid request units".to_string();
                        if update_request.units > 0f64 {
                            let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                            match transactions
                                .inner_join(
                                    sell_orders
                                        .on(schema::open_em::sell_orders::dsl::sell_order_id
                                            .eq(schema::open_em::transactions::dsl::sell_order_id)),
                                )
                                .filter(producer_id.eq(node.node_id))
                                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                                .order_by(schema::open_em::transactions::created_at.asc())
                                .select((Transaction::as_select(), SellOrder::as_select()))
                                .load::<(Transaction, SellOrder)>(connection)
                            {
                                Ok(result_vec) => {
                                    message = "Insufficient available units to produce".to_string();
                                    for (transaction, _) in result_vec {
                                        if transaction.transacted_units - transaction.units_produced
                                            == 0f64
                                        {
                                            continue;
                                        }
                                        if transaction.transacted_units - transaction.units_produced
                                            >= update_request.units
                                        {
                                            match diesel::update(transactions)
                                                .set(units_produced.eq(transaction.units_produced
                                                    + update_request.units))
                                                .filter(
                                                    transaction_id.eq(transaction.transaction_id),
                                                )
                                                .execute(connection)
                                            {
                                                Ok(_) => {
                                                    message = "Units updated".to_string();
                                                    break;
                                                }
                                                Err(_) => {}
                                            }
                                        } else {
                                            match diesel::update(transactions)
                                                .set(
                                                    units_produced.eq(transaction.transacted_units),
                                                )
                                                .filter(
                                                    transaction_id.eq(transaction.transaction_id),
                                                )
                                                .execute(connection)
                                            {
                                                Ok(_) => {
                                                    update_request.units -= transaction
                                                        .transacted_units
                                                        - transaction.units_produced
                                                }
                                                Err(_) => {}
                                            }
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
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RemoveNode {
    node_id: String,
}

#[post(
    "/remove_node",
    format = "application/json",
    data = "<remove_node_request>"
)]
pub fn remove_node(remove_node_request: Json<RemoveNode>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::nodes::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message;
    if claims.user_id != Uuid::nil() {
        match Uuid::parse_str(&*remove_node_request.node_id) {
            Ok(request_node_id) => {
                match diesel::update(nodes)
                    .filter(node_owner.eq(claims.user_id))
                    .filter(node_id.eq(request_node_id))
                    .set(node_active.eq(false))
                    .execute(connection)
                {
                    Ok(_) => message = "Node successfully removed".to_string(),
                    Err(_) => message = "No matching node".to_string(),
                }
            }
            Err(_) => message = "Invalid Node ID".to_string(),
        }
    }

    json!({"status": "ok", "message": message})
}
