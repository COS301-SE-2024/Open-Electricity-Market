use crate::grid::{ToJson, Voltage};
use rocket::serde::json::json;

#[derive(Clone)]
pub struct Transformer {
    pub(crate) id: u32,
    pub(crate) ratio: f32,
    pub(crate) primary_circuit: u32,
    pub(crate) secondary_circuit: u32,
    pub(crate) primary_load: u32,
    pub(crate) secondary_voltage: Voltage,
}

impl ToJson for Transformer {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
                "Ratio" : self.ratio,
        })
        .to_string()
    }
}
