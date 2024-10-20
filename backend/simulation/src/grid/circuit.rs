use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Connection, Load};
use crate::grid::transformer::Transformer;
use crate::grid::{CurrentWrapper, VoltageWrapper};
use rocket::serde::Serialize;
use std::sync::{Arc, Mutex};

use super::{OscilloscopeDetail, Resistance, Voltage};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Circuit {
    pub(crate) id: u32,
    pub(crate) loads: Vec<Load>,
    pub(crate) connections: Vec<Connection>,
    pub(crate) generators: Vec<Generator>,
    pub(crate) transformers: Vec<Arc<Mutex<Transformer>>>,
}

impl Circuit {
    pub(crate) fn set_generater(&mut self, id: u32, max_voltage: f32) {
        let position = self.generators.iter().position(|x| x.id == id).unwrap();
        self.generators[position].max_voltage = max_voltage;
    }

    pub(crate) fn set_consumer(&mut self, id: u32, power: f32) {
        let position = self.loads.iter().position(|x| x.id == id).unwrap();
        match &mut self.loads[position].load_type {
            super::load::LoadType::Consumer(c) => {
                let resitance = (240.0 * 240.0) / power;
                println!("****************************************************************************************************");
                println!("Since I havae an input of {} watt and my sockets are assumed to be at 240 V I set my impedance to {}",power,resitance);
                println!("****************************************************************************************************");

                if resitance > 0.0 {
                    c.resistance = Resistance(resitance);
                }
            }
            super::load::LoadType::TransmissionLine(_) => todo!(),
        }
    }

    pub(crate) fn calculate_ideal_generator_voltages(
        &mut self,
        elapsed_time: f32,
    ) -> VoltageWrapper {
        for gen in self.generators.iter_mut() {
            gen.voltage.voltage.0 = gen.max_voltage
                * f32::sin(gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time);
            gen.voltage.voltage.1 = gen.max_voltage
                * f32::sin(
                    gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time
                        - (2.0 * std::f32::consts::PI / 3.0),
                );
            gen.voltage.voltage.2 = gen.max_voltage
                * f32::sin(
                    gen.frequency * 2.0 * std::f32::consts::PI * elapsed_time
                        - (2.0 * 2.0 * std::f32::consts::PI / 3.0),
                );
            gen.voltage.oscilloscope_detail.amplitude = gen.max_voltage;
            gen.voltage.oscilloscope_detail.frequency = gen.frequency;
        }

        let mut out = VoltageWrapper {
            voltage: Voltage(0.0, 0.0, 0.0),
            oscilloscope_detail: OscilloscopeDetail {
                frequency: 50.0,
                amplitude: 0.0,
                phase: 0.0,
            },
        };

        if !self.generators.is_empty() {
            for gen in self.generators.iter() {
                out = out.add_voltage(gen.voltage.clone())
            }
        }

        for transformer in self.transformers.iter() {
            let transformer = transformer.lock().unwrap();
            if transformer.secondary_circuit == self.id {
                out = out.add_voltage(transformer.secondary_voltage.clone());
            }
        }

        out
    }

    pub(crate) fn calculate_equivalent_impedance(&mut self, frequency: f32, load: usize) -> f32 {
        let mut parallel = vec![];
        let mut series = vec![];
        for con in self.connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *primary == load as u32 {
                        parallel.push(*secondary)
                    }
                }
                Series(primary, secondary) => {
                    if *primary == load as u32 {
                        series.push(*secondary)
                    }
                }
            }
        }

        let mut equivalence = 0.0;
        if !parallel.is_empty() {
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

    pub(crate) fn set_voltages(&mut self, current: CurrentWrapper, frequency: f32, load: usize) {
        let mut parrallel = vec![];
        let mut series = vec![];
        for con in self.connections.iter() {
            match con {
                Parallel(primary, secondary) => {
                    if *primary == load as u32 {
                        parrallel.push(*secondary)
                    }
                }
                Series(primary, secondary) => {
                    if *primary == load as u32 {
                        series.push(*secondary)
                    }
                }
            }
        }

        let mut total = self.loads[load].get_impedance(frequency).0;
        for par in parrallel.iter() {
            total += self.calculate_equivalent_impedance(frequency, *par as usize);
        }

        let factor = self.loads[load].get_impedance(frequency).0 / total;
        self.loads[load].set_voltage(current.scale(factor), frequency);

        for par in parrallel.iter() {
            let factor = self.calculate_equivalent_impedance(frequency, *par as usize) / total;
            self.set_voltages(current.scale(factor), frequency, *par as usize);
        }

        for ser in series {
            self.set_voltages(current.clone(), frequency, ser as usize);
        }
    }

    //Operates under assumption that id's correspond to index
    pub(crate) fn set_transformers_secondary_voltages(&mut self, _frequency: f32) {
        for transformer in self.transformers.iter_mut() {
            let mut transformer = transformer.lock().unwrap();
            if transformer.primary_circuit == self.id {
                let prev_load_id = transformer.primary_load;
                // let prev_load = self.loads[prev_load_id];
                let mut lane_start = 0;
                for con in self.connections.iter() {
                    match con {
                        Parallel(_, _) => {}
                        Series(primary, secondary) => {
                            if *primary == prev_load_id {
                                lane_start = *primary;
                                break;
                            }
                            if *secondary == prev_load_id {
                                lane_start = *primary;
                                break;
                            }
                        }
                    }
                }
                let mut lane = vec![lane_start];
                let mut total_voltage = self.loads[lane_start as usize].get_voltage();

                for con in self.connections.iter() {
                    match con {
                        Parallel(_, _) => {}
                        Series(primary, secondary) => {
                            if *primary == lane_start {
                                lane.push(*secondary);
                                let load = self.loads[*secondary as usize].get_voltage();
                                total_voltage = total_voltage.add_voltage(load);
                            }
                        }
                    }
                }

                for load_id in lane {
                    let load = self.loads[load_id as usize].get_voltage();
                    if load_id == prev_load_id {
                        if transformer.target.is_some() {
                            let target = transformer.target.unwrap();
                            let incomming_voltage = total_voltage.oscilloscope_detail.amplitude;
                            if incomming_voltage != 0.0 {
                                transformer.ratio = target / incomming_voltage;
                            }
                        }

                        transformer.secondary_voltage.voltage.0 =
                            total_voltage.voltage.0 * transformer.ratio;
                        transformer.secondary_voltage.voltage.1 =
                            total_voltage.voltage.1 * transformer.ratio;
                        transformer.secondary_voltage.voltage.2 =
                            total_voltage.voltage.2 * transformer.ratio;
                        transformer.secondary_voltage.oscilloscope_detail.amplitude =
                            total_voltage.oscilloscope_detail.amplitude * transformer.ratio;

                        total_voltage.subtract_voltage(load);
                    }
                }
            }
        }
    }
}
