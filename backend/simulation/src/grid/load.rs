use crate::grid::{Current, Harry, Resistance, ToJson, Voltage};
use rocket::serde::json::json;
#[cfg(test)]
mod tests;

pub enum Connection {
    Parallel(u32,u32),
    Series(u32,u32)
}

pub struct Load {
    pub load_type: LoadType,
    pub id : u32
}

impl Load {
    fn get_resistance(&self) -> Resistance {
        return match &self.load_type {
            LoadType::Consumer(c) => {
                c.resistance.clone()
            }
            LoadType::TransmissionLine(t) => {
                t.resistance.clone()
            }
        }
    }

    fn get_positive_reactance(&self, frequency :f32) -> f32 {
       return std::f32::consts::FRAC_2_PI*frequency*self.get_inductance().0;
    }

    fn get_negative_reactance(&self,frequency :f32) ->f32 {
        return 0.0;
    }

    fn get_inductance(&self) -> Harry {
        return match &self.load_type {
            LoadType::Consumer(_) => {
                Harry(0.0)
            }
            LoadType::TransmissionLine(t) => {
                Harry(t.length * t.inductance_per_meter)
            }
        }
    }

    pub fn get_impedance(&self,frequency :f32) -> Resistance{
        let resistance = self.get_resistance().0;
        let positive_reactance = self.get_positive_reactance(frequency);
        let negative_reactance = self.get_negative_reactance(frequency);
        let reactance = positive_reactance-negative_reactance;
        return Resistance(f32::sqrt(resistance*resistance+reactance*reactance))
    }

    pub fn set_voltage(&mut self,current: Current,frequency :f32){
        match &mut self.load_type {
            LoadType::Consumer(c) => {
                c.voltage.0 = current.0*c.resistance.0;
                c.voltage.1 = current.1*c.resistance.0;
                c.voltage.2 = current.2*c.resistance.0;
            }
            LoadType::TransmissionLine(t) => {
                let impedance = self.get_impedance(frequency);
                t.voltage.0 = current.0*impedance.0;
                t.voltage.1 = current.1*impedance.0;
                t.voltage.2 = current.2*impedance.0;
            }
        }
    }


}

pub enum LoadType {
    Consumer(Consumer),
    TransmissionLine(TransmissionLine)
}

pub struct Consumer {
    pub(crate) id: u32,
    pub(crate) resistance: Resistance,
    pub(crate) voltage: Voltage,
}

impl ToJson for Consumer {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
            "Resistance" : self.resistance.0,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            }
        })
        .to_string()
    }
}

pub struct TransmissionLine {
    pub(crate) id: u32,
    pub(crate) resistance: Resistance,
    pub(crate) voltage: Voltage,
    pub length : f32,
    pub inductance_per_meter :f32
}

impl ToJson for TransmissionLine {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
            "Resistance" : self.resistance.0,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            }
        })
            .to_string()
    }
}