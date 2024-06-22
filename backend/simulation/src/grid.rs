use crate::grid::consumer::Consumer;
use crate::grid::generator::Generator;
use crate::grid::transformer::Transformer;
use crate::grid::transmission_line::TransmissionLine;
use rocket::serde::json::json;

pub mod consumer;
pub mod generator;
pub mod transformer;
pub mod transmission_line;

pub trait ToJson {
    fn to_json(&self) -> String;
}
pub struct Resistance(pub f32);
pub struct Voltage(pub f32, pub f32, pub f32);

pub struct Grid {
    pub(crate) consumers: Vec<Consumer>,
    pub(crate) transmission_lines: Vec<TransmissionLine>,
    pub(crate) generators: Vec<Generator>,
    pub(crate) transformers: Vec<Transformer>,
    pub(crate) started: bool,
}

impl Grid {
    pub(crate) fn get_average_line_voltage(&self) -> String {
        let mut phase1 = 0.0;
        let mut phase2 = 0.0;
        let mut phase3 = 0.0;
        for line in self.transmission_lines.iter() {
            phase1 += line.voltage.0;
            phase2 += line.voltage.1;
            phase3 += line.voltage.2;
        }
        phase1 /= self.transmission_lines.len() as f32;
        phase2 /= self.transmission_lines.len() as f32;
        phase3 /= self.transmission_lines.len() as f32;

        json!({
            "Phase1":phase1,
            "Phase2":phase2,
            "Phase3":phase3
        })
        .to_string()
    }

    pub(crate) fn add_generator(
        &mut self,
        voltage: Voltage,
        max_voltage: f32,
        frequency: f32,
        transmission_line: u32,
    ) -> u32 {
        let id: u32 = self.generators.len() as u32;
        self.generators.push(Generator {
            id,
            voltage,
            max_voltage,
            frequency,
            transmission_line,
        });
        id
    }

    pub(crate) fn add_consumer(
        &mut self,
        resistance: Resistance,
        transmission_line: u32,
        voltage: Voltage,
    ) -> u32 {
        let id: u32 = self.consumers.len() as u32;
        self.consumers.push(Consumer {
            id,
            resistance,
            transmission_line,
            voltage,
        });
        id
    }

    pub(crate) fn update_generator(&mut self, id: u32, max_voltages: f32) {
        let index = self.generators.iter().position(|c| c.id == id);
        match index {
            None => {}
            Some(i) => {
                self.generators[i].max_voltage = max_voltages;
            }
        }
    }

    pub(crate) fn update_consumer(&mut self, id: u32, resistance: Resistance) {
        let index = self.consumers.iter().position(|c| c.id == id);
        match index {
            None => {}
            Some(i) => {
                self.consumers[i].resistance.0 = resistance.0;
            }
        }
    }

    pub(crate) fn update_impedance(&mut self) {
        for line in self.transmission_lines.iter_mut() {
            let index = self
                .consumers
                .iter()
                .position(|c| c.transmission_line == line.id);
            match index {
                None => line.impedance = Resistance(line.resistance.0),
                Some(i) => {
                    line.impedance = Resistance(line.resistance.0 + self.consumers[i].resistance.0)
                }
            }
        }
    }

    pub(crate) fn update_generator_voltages(&mut self, elapsed_time: f32) {
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

    pub(crate) fn sync_voltages(&mut self) {
        // Gens to lines
        for gen in self.generators.iter_mut() {
            let index = self
                .transmission_lines
                .iter()
                .position(|l| l.id == gen.transmission_line);
            match index {
                None => {}
                Some(i) => {
                    self.transmission_lines[i].voltage.0 = gen.voltage.0;
                    self.transmission_lines[i].voltage.1 = gen.voltage.1;
                    self.transmission_lines[i].voltage.2 = gen.voltage.2;
                }
            }
        }
        // Steps
        for trans in self.transformers.iter() {
            let primary_index = self
                .transmission_lines
                .iter()
                .position(|l| l.id == trans.primary);
            let secondary_index = self
                .transmission_lines
                .iter()
                .position(|l| l.id == trans.secondary);
            match primary_index {
                None => {}
                Some(i) => match secondary_index {
                    None => {}
                    Some(j) => {
                        self.transmission_lines[j].voltage.0 =
                            self.transmission_lines[i].voltage.0 * trans.ratio;
                        self.transmission_lines[j].voltage.1 =
                            self.transmission_lines[i].voltage.1 * trans.ratio;
                        self.transmission_lines[j].voltage.2 =
                            self.transmission_lines[i].voltage.2 * trans.ratio;
                    }
                },
            }
        }

        // Lines to consumers
        for line in self.transmission_lines.iter_mut() {
            let index = self
                .consumers
                .iter()
                .position(|c| c.transmission_line == line.id);
            match index {
                None => {}
                Some(i) => {
                    self.consumers[i].voltage.0 = line.voltage.0;
                    self.consumers[i].voltage.1 = line.voltage.1;
                    self.consumers[i].voltage.2 = line.voltage.2;
                }
            }
        }
    }
}

impl ToJson for Grid {
    fn to_json(&self) -> String {
        // consumers : Vec<Consumer>,
        // transmission_lines: Vec<TransmissionLine>,
        // generators : Vec<Generator>,
        // transformers: Vec<Transformer>,
        // started : bool
        let mut consumer_strings: Vec<String> = vec![];
        for consumer in self.consumers.iter() {
            consumer_strings.push(consumer.to_json());
        }

        let mut transmission_line_strings: Vec<String> = vec![];
        for lines in self.transmission_lines.iter() {
            transmission_line_strings.push(lines.to_json());
        }

        let mut generator_strings: Vec<String> = vec![];
        for gen in self.generators.iter() {
            generator_strings.push(gen.to_json());
        }

        let mut transformer_strings: Vec<String> = vec![];
        for trans in self.transformers.iter() {
            transformer_strings.push(trans.to_json());
        }

        json!({"Consumers" : consumer_strings,
            "Transmission Lines" : transmission_line_strings,
            "Generators" : generator_strings,
            "Transformers" : transformer_strings,
            "Started" : self.started
        })
        .to_string()
    }
}
