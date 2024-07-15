use crate::grid::circuit::Circuit;
use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Connection, Consumer, Load, TransmissionLine};
use crate::grid::transformer::Transformer;
use rocket::serde::json::json;

pub mod generator;
pub mod load;
pub mod location;
pub mod transformer;

pub mod circuit;

#[cfg(test)]
mod tests;

pub trait ToJson {
    fn to_json(&self) -> String;
}
#[derive(Clone)]
pub struct Resistance(pub f32);
#[derive(Clone)]
pub struct Voltage(pub f32, pub f32, pub f32);
impl Voltage {
    pub fn add_voltage(&self, other: Voltage) -> Voltage {
        let mut out = other;
        out.0 += self.0;
        out.1 += self.1;
        out.2 += self.2;
        return out;
    }

    pub fn subtract_voltage(&self, other: Voltage) -> Voltage {
        let mut out = self.clone();
        out.0 -= other.0;
        out.1 -= other.1;
        out.2 -= other.2;
        return out;
    }
}

#[derive(Clone)]
pub struct Harry(pub f32);
#[derive(Clone)]
pub struct Current(pub f32, pub f32, pub f32);

impl Current {
    fn ohms_law(voltage: Voltage, resistance: Resistance) -> Current {
        let mut current = Current(0.0, 0.0, 0.0);
        current.0 = voltage.0 / resistance.0;
        current.1 = voltage.1 / resistance.0;
        current.2 = voltage.2 / resistance.0;
        return current;
    }

    fn scale(&self, factor: f32) -> Current {
        return Current(self.0 * factor, self.1 * factor, self.2 * factor);
    }
}

pub struct Grid {
    pub circuits: Vec<Circuit>,
    pub frequency: f32,
    pub(crate) started: bool,
}

impl Grid {
    pub fn connect_load_series(&mut self, new: u32, to: u32, circuit: usize) {
        let mut new_primary = to;
        for con in self.circuits[circuit].connections.iter() {
            match con {
                Parallel(_, _) => {}
                Series(primary, secondary) => {
                    if *secondary == new_primary {
                        new_primary = *primary;
                    }
                }
            }
        }

        self.circuits[circuit]
            .connections
            .push(Series(new_primary, new))
    }

    pub fn connect_load_parallel(&mut self, new: u32, to: u32, circuit: usize) {
        let mut new_primary = to;
        for con in self.circuits[circuit].connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *secondary == new_primary {
                        new_primary = *primary;
                    }
                }
                Series(_, _) => {}
            }
        }
        self.circuits[circuit]
            .connections
            .push(Parallel(new_primary, new))
    }

    fn internal_update(&mut self, elapsed_time: f32, circuit: usize) {
        // Step 1 Update voltages
        let voltage = self.circuits[circuit].calculate_ideal_generator_voltages(elapsed_time);
        // Step 2 Calculate Impedance
        let impedance =
            Resistance(self.circuits[circuit].calculate_equivalent_impedance(self.frequency, 0));
        // Step 3 Calculate current
        let current = Current::ohms_law(voltage, impedance);
        // Step 4 Split resistors (and current) back down
        // Step 5 Determine Voltages
        self.circuits[circuit].set_voltages(current, self.frequency, 0);

        //Step 6 Switch to Connected Circuits
        self.circuits[circuit].set_transformers_secondary_voltages(self.frequency);
    }

    pub fn update(&mut self, elapsed_time: f32) {
        self.internal_update(elapsed_time, 0);
    }
}
