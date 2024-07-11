use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Connection, Consumer, Load, TransmissionLine};
use crate::grid::transformer::Transformer;
use rocket::serde::json::json;

pub mod generator;
pub mod load;
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

pub struct Circuit {
    pub(crate) loads: Vec<Load>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) generators: Vec<Generator>,
    pub(crate) transformers: Vec<Transformer>,
}

impl Circuit {
    fn calculate_ideal_generator_voltages(&mut self, elapsed_time: f32) -> Voltage {
        for gen in self.generators.iter_mut() {
            gen.voltage.0 = gen.max_voltage
                * f32::sin(gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time);
            gen.voltage.1 = gen.max_voltage
                * f32::sin(
                    gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time
                        - (2.0 * std::f32::consts::PI / 3.0),
                );
            gen.voltage.2 = gen.max_voltage
                * f32::sin(
                    gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time
                        - (2.0 * 2.0 * std::f32::consts::PI / 3.0),
                );
        }

        return self.generators[0].voltage.clone();
    }

    fn calculate_equivalent_impedance(&mut self, frequency: f32, load: usize) -> f32 {
        let mut parallel = vec![];
        let mut series = vec![];
        for con in self.connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *primary == load as u32 {
                        parallel.push(secondary.clone())
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
        if parallel.len() > 0 {
            equivalence = 1.0 / self.loads[load].get_impedance(frequency).0;
        }
        for res in parallel {
            equivalence += 1.0 / self.calculate_equivalent_impedance(frequency, res as usize);
        }

        if equivalence != 0.0 {
            equivalence = 1.0 / equivalence;
        }

        if equivalence == 0.0 {
            equivalence += self.loads[load].get_impedance(frequency).0;
        }
        for res in series {
            equivalence += self.calculate_equivalent_impedance(frequency, res as usize);
        }
        equivalence
    }

    fn set_voltages(&mut self, current: Current, frequency: f32, load: usize) {
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

        let mut total = self.loads[load].get_impedance(frequency).0;
        for par in parrallel.iter() {
            total += self.calculate_equivalent_impedance(frequency, par.clone() as usize);
        }

        let factor = self.loads[load].get_impedance(frequency).0 / total;
        self.loads[load].set_voltage(current.scale(factor), frequency);

        for par in parrallel.iter() {
            let factor =
                self.calculate_equivalent_impedance(frequency, par.clone() as usize) / total;
            self.set_voltages(current.scale(factor), frequency, par.clone() as usize);
        }

        for ser in series {
            self.set_voltages(current.clone(), frequency, ser as usize);
        }
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
    }

    pub fn update(&mut self, elapsed_time: f32) {
        self.internal_update(elapsed_time, 0);
    }
}
