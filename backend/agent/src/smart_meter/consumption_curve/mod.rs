use serde::{Deserialize, Serialize};

use crate::curve::Curve;

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
    SolarPanel,
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
            HomeApplianceType::WashingMachine => return 2663.0,
            HomeApplianceType::Router => return 38.0,
            HomeApplianceType::Vacuum => return 44.0,
            HomeApplianceType::Dishwasher => return 1806.0,
            HomeApplianceType::Boiler => return 2664.0,
            HomeApplianceType::HairPurifier => return 22.0,
            HomeApplianceType::SoundSystem => return 17.0,
            HomeApplianceType::Printer3d => return 154.0,
            HomeApplianceType::CoffeeMachine => return 1481.0,
            HomeApplianceType::PhoneCharger => return 86.0,
            HomeApplianceType::Fridge => return 1436.0,
            HomeApplianceType::Radiator => return 1348.0,
            HomeApplianceType::Dehumidifier => return 1989.0,
            HomeApplianceType::MicroWaveOven => return 1712.0,
            HomeApplianceType::Laptop => return 61.0,
            HomeApplianceType::Tv => return 54.0,
            HomeApplianceType::Screen => return 30.0,
            HomeApplianceType::SolarPanel => return 417.0,
            HomeApplianceType::Fan => return 83.0,
            HomeApplianceType::AirConditioner => return 45.0,
            HomeApplianceType::Computer => return 280.0,
            HomeApplianceType::Printer => return 30.0,
            HomeApplianceType::Dryer => return 3267.0,
            HomeApplianceType::Freezer => return 1623.0,
        }
    }
}

#[derive(Deserialize)]
pub struct Period {
    start: f64,
    end: f64,
}

impl Period {
    fn during(&self, time: f64) -> bool {
        return self.start < time && self.end > time;
    }
    fn duration(&self) -> f64 {
        return self.end - self.start;
    }
}

#[derive(Deserialize)]
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
            return self.appliance_type.value();
        } else {
            return 0.0;
        }
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let mut total = 0.0;
        for period in self.on_periods.iter() {
            total += period.duration() * self.appliance_type.value();
        }
        return total;
    }
}

impl HomeAppliance {
    pub fn add_period(&mut self, period: Period) {
        self.on_periods.push(period)
    }
}
