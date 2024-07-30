use crate::grid::circuit::Circuit;
use crate::grid::load::Connection::{Parallel, Series};
use rocket::serde::Serialize;
use serde::Deserialize;

pub mod circuit;
pub mod generator;
pub mod load;
pub mod location;
pub mod transformer;

#[cfg(test)]
mod tests;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Resistance(pub f32);
#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Voltage(pub f32, pub f32, pub f32);
impl Voltage {
    pub fn add_voltage(&self, other: Voltage) -> Voltage {
        let mut out = other;
        out.0 += self.0;
        out.1 += self.1;
        out.2 += self.2;
        out
    }

    pub fn subtract_voltage(&self, other: Voltage) -> Voltage {
        let mut out = self.clone();
        out.0 -= other.0;
        out.1 -= other.1;
        out.2 -= other.2;
        out
    }
}

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct VoltageWrapper {
    pub voltage: Voltage,
    pub oscilloscope_detail: OscilloscopeDetail,
}

impl VoltageWrapper {
    pub fn add_voltage(&self, other: VoltageWrapper) -> VoltageWrapper {
        let mut out = self.clone();
        let voltage = out.voltage;
        out.voltage = voltage.add_voltage(other.voltage);
        out.oscilloscope_detail.amplitude += other.oscilloscope_detail.amplitude;
        out
    }

    pub fn subtract_voltage(&self, other: VoltageWrapper) -> VoltageWrapper {
        let mut out = self.clone();
        let voltage = out.voltage;
        out.voltage = voltage.subtract_voltage(other.voltage);
        out.oscilloscope_detail.amplitude -= other.oscilloscope_detail.amplitude;
        out
    }
}

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct OscilloscopeDetail {
    pub frequency: f32,
    pub amplitude: f32,
    pub phase: f32,
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
        current
    }

    fn scale(&self, factor: f32) -> Current {
        Current(self.0 * factor, self.1 * factor, self.2 * factor)
    }
}
#[derive(Clone)]
pub struct CurrentWrapper {
    pub current: Current,
    pub oscilloscope_detail: OscilloscopeDetail,
}

impl CurrentWrapper {
    fn ohms_law(voltage: VoltageWrapper, resistance: Resistance) -> CurrentWrapper {
        let current = Current::ohms_law(voltage.voltage, resistance.clone());

        CurrentWrapper {
            current,
            oscilloscope_detail: OscilloscopeDetail {
                frequency: voltage.oscilloscope_detail.frequency,
                amplitude: voltage.oscilloscope_detail.amplitude / resistance.0,
                phase: voltage.oscilloscope_detail.phase,
            },
        }
    }

    fn scale(&self, factor: f32) -> CurrentWrapper {
        CurrentWrapper {
            current: self.current.scale(factor),
            oscilloscope_detail: OscilloscopeDetail {
                frequency: self.oscilloscope_detail.frequency,
                amplitude: self.oscilloscope_detail.frequency * factor,
                phase: self.oscilloscope_detail.phase,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Grid {
    pub circuits: Vec<Circuit>,
    pub frequency: f32,
    pub(crate) started: bool,
}

#[derive(Deserialize)]
struct GridInterface {
    circuit: u32,
    generator: u32,
    voltage: f32,
}

#[derive(Serialize)]
pub struct GridStats {
    total_impedance: f32,
    total_generation: f32,
    consumer_count: u32,
    producer_count: u32,
    user_count: u32,
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
        let current = CurrentWrapper::ohms_law(voltage, impedance);
        // Step 4 Split resistors (and current) back down
        // Step 5 Determine Voltages
        self.circuits[circuit].set_voltages(current, self.frequency, 0);

        //Step 6 Switch to Connected Circuits
        self.circuits[circuit].set_transformers_secondary_voltages(self.frequency);
    }

    pub fn update(&mut self, elapsed_time: f32) {
        self.internal_update(elapsed_time, 0);
    }

    pub fn set_generator(&mut self, json: String) {
        let grid_interface: GridInterface = serde_json::from_str(&json).unwrap();
        self.circuits[grid_interface.circuit as usize]
            .set_generater(grid_interface.generator, grid_interface.voltage);
    }

    pub fn get_grid_stats(&self) -> GridStats {
        let mut ouput = GridStats {
            total_impedance: 0.0,
            total_generation: 0.0,
            consumer_count: 0,
            producer_count: 0,
            user_count: 0,
        };

        for cir in self.circuits.iter() {
            for load in cir.loads.iter() {
                ouput.total_impedance += load.get_impedance(self.frequency).0;
                match load.load_type {
                    load::LoadType::Consumer(_) => ouput.consumer_count += 1,
                    load::LoadType::TransmissionLine(_) => {}
                }
            }

            for gen in cir.generators.iter() {
                ouput.total_generation += gen.voltage.oscilloscope_detail.amplitude;
                ouput.producer_count += 1;
            }
        }

        ouput.user_count = ouput.producer_count + ouput.consumer_count;

        ouput
    }
}
