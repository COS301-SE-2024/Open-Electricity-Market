use serde::Serialize;

use crate::{generator::Generator, location::Location, smart_meter::SmartMeter};

#[derive(Serialize)]
pub struct Node {
    pub node_id: String,
    pub location: Location,
    pub smart_meter: SmartMeter,
    pub generator: Generator,
}

impl Node {
    pub fn new(smart_meter: SmartMeter, generator: Generator) -> Node {
        Node {
            node_id: String::new(),
            location: Location::new(),
            smart_meter,
            generator,
        }
    }
}
