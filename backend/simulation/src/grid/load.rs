use crate::grid::location::Location;
use crate::grid::{CurrentWrapper, Harry, Resistance, VoltageWrapper};
use rocket::serde::Serialize;

use super::{OscilloscopeDetail, Voltage};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Connection {
    Parallel(u32, u32),
    Series(u32, u32),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Load {
    pub load_type: LoadType,
    pub id: u32,
}

impl Load {
    fn get_resistance(&self) -> Resistance {
        match &self.load_type {
            LoadType::Consumer(c) => c.resistance.clone(),
            LoadType::TransmissionLine(t) => t.resistance.clone(),
        }
    }

    fn get_positive_reactance(&self, frequency: f32) -> f32 {
        std::f32::consts::FRAC_2_PI * frequency * self.get_inductance().0
    }

    fn get_negative_reactance(&self, _frequency: f32) -> f32 {
        0.0
    }

    fn get_inductance(&self) -> Harry {
        match &self.load_type {
            LoadType::Consumer(_) => Harry(0.0),
            LoadType::TransmissionLine(t) => Harry(t.length * t.inductance_per_meter),
        }
    }

    pub fn get_impedance(&self, frequency: f32) -> Resistance {
        let resistance = self.get_resistance().0;
        let positive_reactance = self.get_positive_reactance(frequency);
        let negative_reactance = self.get_negative_reactance(frequency);
        let reactance = positive_reactance - negative_reactance;
        Resistance(f32::sqrt(resistance * resistance + reactance * reactance))
    }

    pub fn set_voltage(&mut self, current: CurrentWrapper, frequency: f32) {
        let impedance = self.get_impedance(frequency);
        match &mut self.load_type {
            LoadType::Consumer(c) => {
                c.voltage.voltage.0 = current.current.0 * impedance.0;
                c.voltage.voltage.1 = current.current.1 * impedance.0;
                c.voltage.voltage.2 = current.current.2 * impedance.0;
                c.voltage.oscilloscope_detail.frequency = frequency;
                c.voltage.oscilloscope_detail.amplitude =
                    current.oscilloscope_detail.amplitude * impedance.0;
                c.voltage.oscilloscope_detail.phase = current.oscilloscope_detail.phase;
            }
            LoadType::TransmissionLine(t) => {
                t.voltage.voltage.0 = current.current.0 * impedance.0;
                t.voltage.voltage.1 = current.current.1 * impedance.0;
                t.voltage.voltage.2 = current.current.2 * impedance.0;
                t.voltage.oscilloscope_detail.frequency = frequency;
                t.voltage.oscilloscope_detail.amplitude =
                    current.oscilloscope_detail.amplitude * impedance.0;
                t.voltage.oscilloscope_detail.phase = current.oscilloscope_detail.phase;
            }
        }
    }

    pub fn get_voltage(&self) -> VoltageWrapper {
        match &self.load_type {
            LoadType::Consumer(c) => c.voltage.clone(),
            LoadType::TransmissionLine(t) => t.voltage.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum LoadType {
    Consumer(Consumer),
    TransmissionLine(TransmissionLine),
}

impl LoadType {
    pub fn new_transmission_line(length: f32, latitude: f32, longitude: f32) -> LoadType {
        return LoadType::TransmissionLine(TransmissionLine {
            id: 0,
            resistance: Resistance(1.0),
            voltage: VoltageWrapper {
                voltage: Voltage(0.0, 0.0, 0.0),
                oscilloscope_detail: OscilloscopeDetail {
                    frequency: 0.0,
                    amplitude: 0.0,
                    phase: 0.0,
                },
            },
            length,
            inductance_per_meter: 0.5,
            location: Location {
                latitude,
                longitude,
            },
        });
    }
    pub fn new_consumer(latitude: f32, longitude: f32) -> LoadType {
        return LoadType::Consumer(Consumer {
            id: 0,
            resistance: Resistance(1.0),
            voltage: VoltageWrapper {
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
        });
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Consumer {
    pub(crate) id: u32,
    pub(crate) resistance: Resistance,
    pub(crate) voltage: VoltageWrapper,
    pub(crate) location: Location,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TransmissionLine {
    pub(crate) id: u32,
    pub(crate) resistance: Resistance,
    pub(crate) voltage: VoltageWrapper,
    pub length: f32,
    pub inductance_per_meter: f32,
    pub(crate) location: Location,
}
