use crate::grid::location::Location;
use crate::grid::VoltageWrapper;
use rocket::serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Transformer {
    pub(crate) id: u32,
    pub(crate) ratio: f32,
    pub(crate) primary_circuit: u32,
    pub(crate) secondary_circuit: u32,
    pub(crate) primary_load: u32,
    pub(crate) secondary_voltage: VoltageWrapper,
    pub(crate) location: Location,
    pub(crate) target: Option<f32>,
}
