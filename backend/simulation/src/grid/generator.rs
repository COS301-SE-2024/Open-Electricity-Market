use crate::grid::location::Location;
use crate::grid::VoltageWrapper;
use rocket::serde::Serialize;

use super::{OscilloscopeDetail, Voltage};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Generator {
    pub(crate) id: u32,
    pub(crate) voltage: VoltageWrapper,
    pub(crate) max_voltage: f32,
    pub(crate) frequency: f32,
    pub(crate) transmission_line: u32,
    pub(crate) location: Location,
}

impl Generator {
    pub fn new(id: u32, frequency: f32, latitude: f32, longitude: f32) -> Generator {
        Generator {
            id,
            voltage: VoltageWrapper {
                voltage: Voltage(0.0, 0.0, 0.0),
                oscilloscope_detail: OscilloscopeDetail {
                    frequency: 0.0,
                    amplitude: 0.0,
                    phase: 0.0,
                },
            },
            max_voltage: 0.0,
            frequency,
            transmission_line: 0,
            location: Location {
                latitude,
                longitude,
            },
        }
    }
}
