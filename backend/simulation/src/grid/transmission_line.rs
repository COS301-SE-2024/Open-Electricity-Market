use crate::grid::{Resistance, ToJson, Voltage};
use rocket::serde::json::json;

pub struct TransmissionLine {
    pub(crate) id: u32,
    pub(crate) resistance: Resistance,
    pub(crate) impedance: Resistance,
    pub(crate) voltage: Voltage,
}

impl ToJson for TransmissionLine {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
            "Resistance" : self.resistance.0,
            "Impedance" : self.impedance.0,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            }
        })
        .to_string()
    }
}
