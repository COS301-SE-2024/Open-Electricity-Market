use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AddFundDetails {
    pub funds: f64,
}

#[derive(Deserialize)]
pub struct AddFundResult {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginDetail {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterDetail {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SessionWrapper {
    pub token: String,
}

#[derive(Deserialize)]
pub struct LoginResult {
    pub data: SessionWrapper,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct RegisterResult {
    pub data: SessionWrapper,
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct NodeDetail {
    pub name: String,
    pub location_x: f32,
    pub location_y: f32,
}

#[derive(Deserialize)]
pub struct NodeResult {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct GetNodeDetail {
    pub limit: u32,
}

#[derive(Deserialize)]
pub struct NodeWrapper {
    pub name: String,
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct GetNodeResult {
    pub data: Vec<NodeWrapper>,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub credit: f64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UserDetailResult {
    pub data: UserData,
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct NodeDetailsDetails {
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct NodeDetailsData {
    pub location_x: f32,
    pub location_y: f32,
    pub name: String,
    pub node_id: String,
    pub units_to_consume: f64,
    pub units_to_produce: f64,
}

#[derive(Deserialize)]
pub struct NodeDetailsResult {
    pub data: NodeDetailsData,
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct UpdateUnitsConsumedDetails {
    pub units: f64,
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct UpdateUnitsConsumeResult {
    pub message: String,
    pub status: String,
}
#[derive(Serialize)]
pub struct UpdateUnitsProducedDetails {
    pub units: f64,
    pub node_id: String,
}

#[derive(Deserialize)]
pub struct UpdateUnitsProducedResult {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct VoltageUpdateDetail {
    pub circuit: u32,
    pub generator: u32,
    pub power: f32,
}

#[derive(Deserialize)]
pub struct VoltageUpdateResult {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ImpedanceUpdateDetail {
    pub circuit: u32,
    pub consumer: u32,
    pub power: f32,
}

#[derive(Deserialize)]
pub struct ImpedanceUpdateResult {
    pub message: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct GetPriceData {
    pub price: f64,
}

#[derive(Deserialize)]
pub struct GetPriceResult {
    pub data: GetPriceData,
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct PlaceBuyOrderDetail {
    pub node_id: String,
    pub min_price: f64,
    pub max_price: f64,
    pub units: f64,
}

#[derive(Deserialize)]
pub struct BuyOrderResult {
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct PlaceSellOrderDetail {
    pub node_id: String,
    pub min_price: f64,
    pub max_price: f64,
    pub units: f64,
}

#[derive(Deserialize)]
pub struct SellOrderResult {
    pub message: String,
    pub status: String,
}
