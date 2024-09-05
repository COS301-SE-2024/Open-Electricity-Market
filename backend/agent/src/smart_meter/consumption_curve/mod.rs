use serde::{Deserialize, Serialize};

use crate::{curve::Curve, period::Period};

#[derive(Serialize, Deserialize)]
pub enum HomeApplianceType {
    WashingMachine,
    Router,
    Vacuum,
    Dishwasher,
    Boiler,
    HairPurifier,
    SoundSystem,
    Printer3d,
    CoffeeMachine,
    PhoneCharger,
    Fridge,
    Radiator,
    Dehumidifier,
    MicroWaveOven,
    Laptop,
    Tv,
    Screen,
    Fan,
    AirConditioner,
    Computer,
    Printer,
    Dryer,
    Freezer,
}

#[derive(Deserialize, Serialize)]
pub struct HomeAppliance {
    appliance_type: HomeApplianceType,
    
}

impl Curve for HomeAppliance {
    fn sample(&mut self, time: f64) -> f64 {
        let time = time %  86400.0;
        let sample = 0.0;

        sample 
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let total = 0.0;

        total
    }
}


