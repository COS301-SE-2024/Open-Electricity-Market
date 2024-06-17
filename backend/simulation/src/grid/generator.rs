use rocket::serde::json::json;
use crate::grid::{ToJson, Voltage};

pub struct Generator {
    pub(crate) id : u32,
    pub(crate) voltage: Voltage,
    pub(crate) max_voltage:f32,
    pub(crate) frequency :f32,
    pub(crate) transmission_line: u32,
}

impl ToJson for Generator {
    fn to_json(&self) -> String{
        json!({ "ID" : self.id,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            },
            "Max Voltage" : self.max_voltage,
            "Frequency" : self.frequency,
            "Connected Transmission Line" : self.transmission_line
        }).to_string()
    }
}