use crate::models::{BuyOrder, NewNodeModel, Node, SellOrder, Transaction};
use crate::user_management::Claims;
use crate::{establish_connection, schema, TRANSACTION_LIFETIME};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use rocket::serde::{
    json::{serde_json::json, Json, Value},
    Deserialize, Serialize,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddNodeReq<'r> {
    name: &'r str,
    location_x: f64,
    location_y: f64,
}

#[post("/add_node", format = "application/json", data = "<add_node_req>")]
pub fn add_node(add_node_req: Json<AddNodeReq<'_>>, claims: Claims) -> Value {
    use crate::schema::open_em::nodes;

    let connection = &mut establish_connection();

    let new_node_insert = NewNodeModel {
        node_owner: Uuid::parse_str(&*claims.user_id).unwrap(),
        location_x: add_node_req.location_x,
        location_y: add_node_req.location_y,
        name: add_node_req.name,
    };

    match diesel::insert_into(nodes::table)
        .values(&new_node_insert)
        .execute(connection)
    {
        Ok(_) => json!({"status": "ok", "message": "New Node Added".to_string()}),
        Err(_) => json!({"status": "error", "message": "Something went wrong".to_string()}),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetNodesReq {
    limit: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ShortNodeRet {
    node_id: String,
    name: String,
}

#[post(
    "/get_nodes",
    format = "application/json",
    data = "<get_nodes_request>"
)]
pub fn get_nodes(get_nodes_request: Json<GetNodesReq>, claims: Claims) -> Value {
    use crate::schema::open_em::nodes::dsl::*;

    let connection = &mut establish_connection();

    let mut node_list: Vec<ShortNodeRet> = vec![];

    match Uuid::parse_str(&*claims.user_id) {
        Ok(claim_user_id) => {
            match nodes
                .filter(node_owner.eq(claim_user_id))
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
                    json!({"status": "ok",
                        "message": "List of nodes successfully retrieved".to_string(),
                        "data": node_list
                    })
                }
                Err(_) => json!({"status": "ok",
                    "message": "Something went wrong".to_string(),
                    "data": node_list
                }),
            }
        }
        Err(_) => json!({"status": "error",
            "message": "Invalid User ID".to_string(),
            "data": node_list
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NodeDetails {
    node_id: String,
    name: String,
    location_x: f64,
    location_y: f64,
    units_to_produce: f64,
    units_to_consume: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NodeDetailsReq {
    node_id: String,
}

#[post(
    "/node_details",
    format = "application/json",
    data = "<node_details_request>"
)]
pub fn node_details(node_details_request: Json<NodeDetailsReq>, claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let mut data = NodeDetails {
        node_id: "".to_string(),
        name: "".to_string(),
        location_x: 0.0,
        location_y: 0.0,
        units_to_produce: 0.0,
        units_to_consume: 0.0,
    };

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string(), "data": data});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*node_details_request.node_id);
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
            data.node_id = String::from(node.node_id);
            data.name = node.name.clone();
            data.location_x = node.location_x;
            data.location_y = node.location_y;

            let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

            match transactions
                .inner_join(
                    sell_orders.on(schema::open_em::sell_orders::dsl::sell_order_id
                        .eq(schema::open_em::transactions::dsl::sell_order_id)),
                )
                .filter(producer_id.eq(node.node_id))
                .filter(schema::open_em::sell_orders::active.eq(true))
                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                    "SUM(transacted_units - units_produced)",
                ))
                .first::<f64>(connection)
            {
                Ok(result) => {
                    data.units_to_produce = result;
                }
                Err(_) => {}
            };

            match transactions
                .inner_join(
                    buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                        .eq(schema::open_em::transactions::dsl::buy_order_id)),
                )
                .filter(consumer_id.eq(node.node_id))
                .filter(schema::open_em::buy_orders::dsl::active.eq(true))
                .filter(schema::open_em::transactions::created_at.gt(timestamp))
                .select(diesel::dsl::sql::<diesel::sql_types::Double>(
                    "SUM(transacted_units - units_consumed)",
                ))
                .first::<f64>(connection)
            {
                Ok(result) => {
                    data.units_to_consume = result;
                }
                Err(_) => {}
            };
            json!({"status": "ok",
                "message": "Node details retrieved succesfully".to_string(),
                "data": data
            })
        }
        Err(_) => json!({"status": "error",
            "message": "No matching node".to_string(),
            "data": data
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateUnits {
    units: f64,
    node_id: String,
}

#[post(
    "/update_consumed_units",
    format = "application/json",
    data = "<update_request>"
)]
pub fn update_consumed_units(mut update_request: Json<UpdateUnits>, claims: Claims) -> Value {
    use crate::schema::open_em::buy_orders::dsl::*;
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*update_request.node_id);
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
            if update_request.units > 0f64 {
                let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                match transactions
                    .inner_join(
                        buy_orders.on(schema::open_em::buy_orders::dsl::buy_order_id
                            .eq(schema::open_em::transactions::dsl::buy_order_id)),
                    )
                    .filter(consumer_id.eq(node.node_id))
                    .filter(active.eq(true))
                    .filter(schema::open_em::transactions::created_at.gt(timestamp))
                    .order_by(schema::open_em::transactions::created_at.asc())
                    .select((Transaction::as_select(), BuyOrder::as_select()))
                    .load::<(Transaction, BuyOrder)>(connection)
                {
                    Ok(result_vec) => {
                        for (transaction, _) in result_vec {
                            if transaction.transacted_units - transaction.units_consumed == 0f64 {
                                continue;
                            }
                            if transaction.transacted_units - transaction.units_consumed
                                >= update_request.units
                            {
                                match diesel::update(transactions)
                                    .set(
                                        units_consumed
                                            .eq(transaction.units_consumed + update_request.units),
                                    )
                                    .filter(transaction_id.eq(transaction.transaction_id))
                                    .execute(connection)
                                {
                                    Ok(_) => {
                                        return json!({"status": "ok",
                                            "message": "Units updated".to_string()})
                                    }
                                    Err(_) => {}
                                }
                            } else {
                                match diesel::update(transactions)
                                    .set(units_consumed.eq(transaction.transacted_units))
                                    .filter(transaction_id.eq(transaction.transaction_id))
                                    .execute(connection)
                                {
                                    Ok(_) => {
                                        update_request.units -= transaction.transacted_units
                                            - transaction.units_consumed
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        return json!({"status": "error", "message": "Insufficient units available to consume".to_string()});
                    }
                    Err(_) => {
                        return json!({"status": "error", "message": "Insufficient units available to consume".to_string()})
                    }
                }
            }
            json!({"status": "error", "message": "Invalid request units".to_string()})
        }
        Err(_) => {
            json!({"status": "error", "message": "No matching node".to_string()})
        }
    }
}

#[post(
    "/update_produced_units",
    format = "application/json",
    data = "<update_request>"
)]
pub fn update_produced_units(mut update_request: Json<UpdateUnits>, claims: Claims) -> Value {
    use crate::schema::open_em::nodes::dsl::*;
    use crate::schema::open_em::sell_orders::dsl::*;
    use crate::schema::open_em::transactions::dsl::*;

    let connection = &mut establish_connection();
    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*update_request.node_id);
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
            if update_request.units > 0f64 {
                let timestamp = Utc::now() - Duration::hours(TRANSACTION_LIFETIME);

                match transactions
                    .inner_join(
                        sell_orders.on(schema::open_em::sell_orders::dsl::sell_order_id
                            .eq(schema::open_em::transactions::dsl::sell_order_id)),
                    )
                    .filter(producer_id.eq(node.node_id))
                    .filter(active.eq(true))
                    .filter(schema::open_em::transactions::created_at.gt(timestamp))
                    .order_by(schema::open_em::transactions::created_at.asc())
                    .select((Transaction::as_select(), SellOrder::as_select()))
                    .load::<(Transaction, SellOrder)>(connection)
                {
                    Ok(result_vec) => {
                        for (transaction, _) in result_vec {
                            if transaction.transacted_units - transaction.units_produced == 0f64 {
                                continue;
                            }
                            if transaction.transacted_units - transaction.units_produced
                                >= update_request.units
                            {
                                match diesel::update(transactions)
                                    .set(
                                        units_produced
                                            .eq(transaction.units_produced + update_request.units),
                                    )
                                    .filter(transaction_id.eq(transaction.transaction_id))
                                    .execute(connection)
                                {
                                    Ok(_) => {
                                        return json!({"status": "ok",
                                            "message": "Units updated".to_string()})
                                    }
                                    Err(_) => {}
                                }
                            } else {
                                match diesel::update(transactions)
                                    .set(units_produced.eq(transaction.transacted_units))
                                    .filter(transaction_id.eq(transaction.transaction_id))
                                    .execute(connection)
                                {
                                    Ok(_) => {
                                        update_request.units -= transaction.transacted_units
                                            - transaction.units_produced
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        return json!({"status": "error",
                            "message": "Insufficient available units to produce".to_string()});
                    }
                    Err(_) => {}
                }
            }
            json!({"status": "error", "message": "Invalid request units".to_string()})
        }
        Err(_) => {
            json!({"status": "error", "message": "No matching node".to_string()})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RemoveNode {
    node_id: String,
}

#[post(
    "/remove_node",
    format = "application/json",
    data = "<remove_node_request>"
)]
pub fn remove_node(remove_node_request: Json<RemoveNode>, claims: Claims) -> Value {
    use crate::schema::open_em::nodes::dsl::*;

    let connection = &mut establish_connection();

    let user_id_parse = Uuid::parse_str(&*claims.user_id);
    if user_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid User ID".to_string()});
    }
    let claim_user_id = user_id_parse.unwrap();

    let node_id_parse = Uuid::parse_str(&*remove_node_request.node_id);
    if node_id_parse.is_err() {
        return json!({"status": "error", "message": "Invalid Node ID".to_string()});
    }
    let request_node_id = node_id_parse.unwrap();

    match diesel::update(nodes)
        .filter(node_owner.eq(claim_user_id))
        .filter(node_id.eq(request_node_id))
        .set(node_active.eq(false))
        .execute(connection)
    {
        Ok(_) => json!({"status": "ok", "message": "Node successfully removed".to_string()}),
        Err(_) => json!({"status": "ok", "message": "No matching node".to_string()}),
    }
}
