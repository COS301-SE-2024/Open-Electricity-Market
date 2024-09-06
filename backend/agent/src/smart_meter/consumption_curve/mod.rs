use diesel::define_sql_function;
use diesel::sql_types::Float8;
use diesel::sql_types::Text;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};

use crate::curve::Curve;
use crate::establish_connection;

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
    pub fn to_string(&self) -> String {
        return match self {
            HomeApplianceType::WashingMachine => String::from("washing_machine"),
            HomeApplianceType::Router => String::from("washing_machine"),
            HomeApplianceType::Vacuum => String::from("vacuum"),
            HomeApplianceType::Dishwasher => String::from("dish_washer"),
            HomeApplianceType::Boiler => String::from("boiler"),
            HomeApplianceType::HairPurifier => String::from("hair_purifier"),
            HomeApplianceType::SoundSystem => String::from("sound_system"),
            HomeApplianceType::Printer3d => String::from("printer_3D"),
            HomeApplianceType::CoffeeMachine => String::from("coffee_machine"),
            HomeApplianceType::PhoneCharger => String::from("phone_charger"),
            HomeApplianceType::Fridge => String::from("fridge"),
            HomeApplianceType::Radiator => String::from("radiator"),
            HomeApplianceType::Dehumidifier => String::from("dehumifer"),
            HomeApplianceType::MicroWaveOven => String::from("micro_wave_oven"),
            HomeApplianceType::Laptop => String::from("laptop"),
            HomeApplianceType::Tv => String::from("tv"),
            HomeApplianceType::Screen => String::from("screen"),
            HomeApplianceType::Fan => String::from("fan"),
            HomeApplianceType::AirConditioner => String::from("air_conditioner"),
            HomeApplianceType::Computer => String::from("computer"),
            HomeApplianceType::Printer => String::from("printer"),
            HomeApplianceType::Dryer => String::from("dryer"),
            HomeApplianceType::Freezer => String::from("freezer"),
        };
    }
}

#[derive(Deserialize, Serialize)]
pub struct HomeAppliance {
    pub appliance_type: HomeApplianceType,
}

define_sql_function! { fn sample_appliance(value: Float8, appliance: Text) -> Float8;}
define_sql_function! { fn total_appliance(appliance: Text) -> Float8;}

impl Curve for HomeAppliance {
    fn sample(&mut self, time: f64) -> f64 {
        let time = time % 86400.0;
        let conn = &mut establish_connection();
        let sample_appliance = sample_appliance(time, self.appliance_type.to_string());
        let sample: f64 = diesel::select(sample_appliance).first(conn).unwrap();

        sample
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let conn = &mut establish_connection();
        let total_appliance = total_appliance(self.appliance_type.to_string());
        let total = diesel::select(total_appliance).first(conn).unwrap();

        total
    }

    fn get_appliance_list_if_possible(&mut self) -> Vec<String> {
        return vec![self.appliance_type.to_string()];
    }
}
