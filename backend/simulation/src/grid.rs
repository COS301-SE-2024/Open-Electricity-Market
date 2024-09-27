use std::sync::{Arc, Mutex};

use crate::grid::circuit::Circuit;
use crate::grid::load::Connection::{Parallel, Series};

use rocket::form::validate::Contains;
use rocket::serde::{Deserialize, Serialize};

use self::generator::Generator;
use self::load::{Load, LoadType};
use self::location::Location;
use self::transformer::Transformer;

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
                amplitude: self.oscilloscope_detail.amplitude * factor,
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
#[serde(crate = "rocket::serde")]
pub struct GeneratorInterface {
    pub circuit: u32,
    pub generator: u32,
    power: f32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConsumerInterface {
    pub circuit: u32,
    pub consumer: u32,
    power: f32,
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
    pub fn create_consumer(&mut self, latitude: f32, longitude: f32) -> (u32, u32) {
        let mut nearest_circuit = 0;
        let mut nearest_transmission_line = 0;
        let mut found = false;
        let mut neares_distance = f32::INFINITY;

        for circuit in self.circuits.iter() {
            let circuit_id = circuit.id;
            for load in circuit.loads.iter() {
                match &load.load_type {
                    load::LoadType::Consumer(_) => {}
                    load::LoadType::TransmissionLine(line) => {
                        let line_latitude = line.location.latitude;
                        let line_longitude = line.location.longitude;

                        let y = latitude - line_latitude;
                        let x = longitude - line_longitude;

                        let dist = f32::sqrt(x * x + y * y);

                        if dist < neares_distance {
                            neares_distance = dist;
                            nearest_transmission_line = line.id;
                            nearest_circuit = circuit_id;
                            found = true;
                        }
                    }
                }
            }
        }

        if !found {
            neares_distance = 5.0;
        }

        let tl_id = self.circuits[nearest_circuit as usize].loads.len();
        self.circuits[nearest_circuit as usize].loads.push(Load {
            load_type: LoadType::new_transmission_line(neares_distance, latitude, longitude),
            id: tl_id as u32,
        });

        if found {
            self.connect_load_series(
                tl_id as u32,
                nearest_transmission_line,
                nearest_circuit as usize,
            )
        }

        let cn_id = 0;

        let new_circuit_id = self.circuits.len();

        self.circuits.push(Circuit {
            id: new_circuit_id as u32,
            loads: vec![],
            connections: vec![],
            generators: vec![],
            transformers: vec![],
        });

        let transformer = Transformer {
            id: 0,
            ratio: 1.0,
            primary_circuit: nearest_circuit,
            secondary_circuit: new_circuit_id as u32,
            primary_load: tl_id as u32,
            secondary_voltage: VoltageWrapper {
                voltage: Voltage(0.0, 0.0, 0.0),
                oscilloscope_detail: OscilloscopeDetail {
                    frequency: 0.0,
                    amplitude: 0.0,
                    phase: 0.0,
                },
            },
            location: Location {
                latitude,
                longitude,
            },
            target: Some(240.0),
        };
        let trans_ref = Arc::new(Mutex::new(transformer));

        self.circuits[nearest_circuit as usize]
            .transformers
            .push(trans_ref.clone());
        self.circuits[new_circuit_id]
            .transformers
            .push(trans_ref.clone());

        self.circuits[new_circuit_id].loads.push(Load {
            load_type: LoadType::new_consumer(latitude, longitude),
            id: cn_id as u32,
        });

        // self.connect_load_series(cn_id as u32, tl_id as u32, nearest_circuit as usize);

        (cn_id as u32, new_circuit_id as u32)
    }

    pub fn create_producer(&mut self, latitude: f32, longitude: f32) -> (u32, u32) {
        let mut nearest_circuit = 0;
        let mut nearst_distance = f32::INFINITY;

        for circuit in self.circuits.iter() {
            let id = circuit.id;
            for transformer in circuit.transformers.iter() {
                let transformer = transformer.lock().unwrap();
                let trans_latitude = transformer.location.latitude;
                let trans_longitude = transformer.location.longitude;

                let y = trans_latitude - latitude;
                let x = trans_longitude - longitude;

                let dist = f32::sqrt(x * x + y * y);

                if dist < nearst_distance {
                    nearest_circuit = id;
                    nearst_distance = dist
                }
            }
        }

        let index = self
            .circuits
            .iter()
            .position(|cur| cur.id == nearest_circuit)
            .unwrap();

        let id = self.circuits[index].generators.len();

        self.circuits[index]
            .generators
            .push(Generator::new(id as u32, 50.0, latitude, longitude));

        (nearest_circuit, id as u32)
    }

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

    fn internal_update(&mut self, elapsed_time: f32, circuit: usize, visited: &mut Vec<usize>) {
        // Step 1 Update voltages
        let voltage = self.circuits[circuit].calculate_ideal_generator_voltages(elapsed_time);

        // Step 2 Calculate Impedance
        let impedance =
            Resistance(self.circuits[circuit].calculate_equivalent_impedance(self.frequency, 0));
        // Step 3 Calculate current
        let current = CurrentWrapper::ohms_law(voltage.clone(), impedance.clone());
        // Step 4 Split resistors (and current) back down
        // Step 5 Determine Voltages
        self.circuits[circuit].set_voltages(current, self.frequency, 0);

        //Step 6 Switch to Connected Circuits
        self.circuits[circuit].set_transformers_secondary_voltages(self.frequency);

        let mut next = vec![];

        for trans in self.circuits[circuit].transformers.iter() {
            let transformer = trans.lock().unwrap();
            next.push(transformer.secondary_circuit as usize);
        }

        for n in next {
            if !visited.contains(n) {
                visited.push(n);
                // println!("circuit {n}");
                self.internal_update(elapsed_time, n, visited);
            }
        }
    }

    pub fn update(&mut self, elapsed_time: f32) {
        // println!("Update");
        let mut visited = vec![];
        self.internal_update(elapsed_time, 0, &mut visited);
    }

    pub fn set_consumer(&mut self, grid_interface: ConsumerInterface) {
        self.circuits[grid_interface.circuit as usize]
            .set_consumer(grid_interface.consumer, grid_interface.power);
    }

    pub fn set_generator(&mut self, grid_interface: GeneratorInterface) {
        let r = self.circuits[grid_interface.circuit as usize]
            .calculate_equivalent_impedance(self.frequency, 0);

        self.circuits[grid_interface.circuit as usize].set_generater(
            grid_interface.generator,
            f32::sqrt(grid_interface.power * r),
        );
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
