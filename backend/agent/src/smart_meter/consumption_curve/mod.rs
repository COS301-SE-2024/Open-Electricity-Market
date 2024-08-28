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

impl HomeApplianceType {
    fn value(&self) -> f64 {
        match self {
            HomeApplianceType::WashingMachine => 2663.0,
            HomeApplianceType::Router => 38.0,
            HomeApplianceType::Vacuum => 44.0,
            HomeApplianceType::Dishwasher => 1806.0,
            HomeApplianceType::Boiler => 2664.0,
            HomeApplianceType::HairPurifier => 22.0,
            HomeApplianceType::SoundSystem => 17.0,
            HomeApplianceType::Printer3d => 154.0,
            HomeApplianceType::CoffeeMachine => 1481.0,
            HomeApplianceType::PhoneCharger => 86.0,
            HomeApplianceType::Fridge => 1436.0,
            HomeApplianceType::Radiator => 1348.0,
            HomeApplianceType::Dehumidifier => 1989.0,
            HomeApplianceType::MicroWaveOven => 1712.0,
            HomeApplianceType::Laptop => 61.0,
            HomeApplianceType::Tv => 54.0,
            HomeApplianceType::Screen => 30.0,
            HomeApplianceType::Fan => 83.0,
            HomeApplianceType::AirConditioner => 45.0,
            HomeApplianceType::Computer => 280.0,
            HomeApplianceType::Printer => 30.0,
            HomeApplianceType::Dryer => 3267.0,
            HomeApplianceType::Freezer => 1623.0,
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct HomeAppliance {
    appliance_type: HomeApplianceType,
    on_periods: Vec<Period>,
}

impl Curve for HomeAppliance {
    fn sample(&mut self, time: f64) -> f64 {
        let time = time % 64.0;
        let mut on = false;
        for period in self.on_periods.iter() {
            if period.during(time) {
                on = true;
                break;
            }
        }
        if on {
            self.appliance_type.value()
        } else {
            0.0
        }
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let mut total = 0.0;
        for period in self.on_periods.iter() {
            total += period.duration() * self.appliance_type.value();
        }
        total
    }
}

impl HomeAppliance {
    pub fn add_period(&mut self, period: Period) {
        self.on_periods.push(period)
    }
}
