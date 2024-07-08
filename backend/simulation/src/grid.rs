use std::thread::current;
use crate::grid::load::{Connection, Consumer, Load, TransmissionLine};
use crate::grid::generator::Generator;
use crate::grid::transformer::Transformer;
use rocket::serde::json::json;
use crate::grid::load::Connection::{Parallel, Series};

pub mod load;
pub mod generator;
pub mod transformer;

#[cfg(test)]
mod tests;

pub trait ToJson {
    fn to_json(&self) -> String;
}
pub struct Resistance(pub f32);
pub struct Voltage(pub f32, pub f32, pub f32);

pub struct Grid {
    pub(crate) load: Load,
    pub(crate) connections: Vec<Connection>,
    pub(crate) generators: Vec<Generator>,
    pub(crate) transformers: Vec<Transformer>,
    pub(crate) started: bool,
}

impl Grid {
    fn calculate_ideal_generator_voltages(&mut self,elapsed_time: f32) {
        for gen in self.generators.iter_mut() {
            gen.voltage.0 = gen.max_voltage
                * f32::sin(gen.frequency * std::f32::consts::FRAC_2_PI * elapsed_time);
            gen.voltage.1 = gen.max_voltage
                * f32::sin(
                gen.frequency * std::f32::consts::FRAC_2_PI * elapsed_time
                    - (std::f32::consts::FRAC_2_PI / 3.0),
            );
            gen.voltage.2 = gen.max_voltage
                * f32::sin(
                gen.frequency * std::f32::consts::FRAC_2_PI * elapsed_time
                    - (2.0 * std::f32::consts::FRAC_2_PI / 3.0),
            );
        }
    }

    pub fn connect_load_series(&mut self, new: u32,to: u32) {
        for con in self.connections.iter() {
            match con {
                Parallel(_, _) => {}
                Series(primary, secondary) => {
                    
                }
            }
        }
    }

    pub fn update(&mut self,elapsed_time: f32){
        // Step 1 Update voltages
        self.calculate_ideal_generator_voltages(elapsed_time);
        // Step 2 Work out resistor equivalences

        // Step 3 Calculate current

        // Step 4 Split resistors (and current) back down

        // Step 5 Determine Voltages
    }
}
