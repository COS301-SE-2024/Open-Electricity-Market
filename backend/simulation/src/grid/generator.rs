use crate::grid::location::Location;
use crate::grid::VoltageWrapper;
use rocket::serde::Serialize;

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
