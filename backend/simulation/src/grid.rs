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
#[derive(Clone)]
pub struct Resistance(pub f32);
#[derive(Clone)]
pub struct Voltage(pub f32, pub f32, pub f32);
#[derive(Clone)]
pub struct Harry(pub f32);



struct Circuit {
    pub(crate) loads: Vec<Load>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) generators: Vec<Generator>,
    pub(crate) transformers: Vec<Transformer>,
}

impl Circuit {
    fn calculate_ideal_generator_voltages(&mut self,elapsed_time: f32) -> Voltage {
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

        return self.generators[0].voltage.clone();
    }

    fn calculate_equivalent_resistance(&mut self, load : usize) ->f32 {
        let mut parrallel = vec![];
        let mut series = vec![];
        for con in self.connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *primary == load as u32 {
                        parrallel.push(secondary.clone())
                    }
                }
                Series(primary, secondary) => {
                    if *primary == load as u32 {
                        series.push(secondary.clone())
                    }
                }
            }
        }


        let mut equivalence = 0.0;
        if parrallel.len() > 0 {
            equivalence = 1.0/self.loads[load].get_resistance().0;
        }
        for res in parrallel {
            equivalence += 1.0/self.calculate_equivalent_resistance(res as usize);
        }
        equivalence = 1.0/ equivalence;

        if equivalence == 0.0 {
            equivalence += self.loads[load].get_resistance().0;
        }
        for res in series {
            equivalence += self.calculate_equivalent_resistance(res as usize);
        }
        equivalence
    }

    fn calculate_positive_reactance(&mut self, frequency:f32) ->f32 {
        let mut inductance = 0.0;
        for load in self.loads.iter() {
            inductance += load.get_inductance().0
        }
        return std::f32::consts::FRAC_2_PI*frequency*inductance;
    }

    fn calculate_negative_reactance(&mut self,frequency:f32) ->f32 {
        return 0.0;
    }
}

pub struct Grid {
    pub circuits : Vec<Circuit>,
    pub(crate) started: bool,
}

impl Grid {


    pub fn connect_load_series(&mut self, new: u32,to: u32,circuit: usize) {
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

        self.circuits[circuit].connections.push(Series(new_primary,new))
    }


    pub fn connect_load_parallel(&mut self, new: u32,to: u32,circuit: usize) {
        let mut new_primary = to;
        for con in self.circuits[circuit].connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *secondary == new_primary {
                        new_primary = *primary;
                    }
                }
                Series(_,_) => {}
            }
        }
        self.circuits[circuit].connections.push(Parallel(new_primary,new))
    }


    fn internal_update(&mut self,elapsed_time: f32,circuit: usize){
        // Step 1 Update voltages
        let voltage = self.circuits[circuit].calculate_ideal_generator_voltages(elapsed_time);
        // Step 2 Calculate Impedance
            // Step 2.1 Work out resistance
            let resistance = self.circuits[circuit].calculate_equivalent_resistance(0);
            // Step 2.2 Work our reactance
            let reactance = self.circuits[circuit].calculate_positive_reactance(50.0)-self.circuits[circuit].calculate_negative_reactance(50.0);
        let impedance = f32::sqrt(resistance*resistance+reactance*reactance);

        // Step 3 Calculate current

        // Step 4 Split resistors (and current) back down

        // Step 5 Determine Voltages
    }

    pub fn update (&mut self,elapsed_time: f32){
        self.internal_update(elapsed_time,0);
    }
}
