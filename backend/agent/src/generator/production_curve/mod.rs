use serde::{Deserialize, Serialize};

use crate::{curve::Curve, period::Period};

#[derive(Serialize, Deserialize)]
pub enum SolarPanelType {
    Home,
    Industrial,
}

//https://www.yesenergysolutions.co.uk/advice/how-much-energy-solar-panels-produce-home
impl SolarPanelType {
    pub fn value(&self) -> f64 {
        match self {
            SolarPanelType::Home => 325.0,
            SolarPanelType::Industrial => 375.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum WindTurbineType {
    Small,
    Medium,
    Large,
}

//https://www.energy.gov/eere/articles/wind-turbines-bigger-better
impl WindTurbineType {
    pub fn value(&self) -> f64 {
        match self {
            WindTurbineType::Small => 700_000.0,
            WindTurbineType::Medium => 2_000_000.0,
            WindTurbineType::Large => 3_200_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum NuclearReactTypes {
    PWR,
    BWR,
    PHWR,
    LWGR,
    AGR,
    FNR,
    HTGR,
}

//https://world-nuclear.org/information-library/nuclear-fuel-cycle/nuclear-power-reactors/nuclear-power-reactors
impl NuclearReactTypes {
    pub fn value(&self) -> f64 {
        match self {
            NuclearReactTypes::PWR => 956_000_000.0,
            NuclearReactTypes::BWR => 1_015_000_000.0,
            NuclearReactTypes::PHWR => 520_000_000.0,
            NuclearReactTypes::LWGR => 650_000_000.0,
            NuclearReactTypes::AGR => 580_000_000.0,
            NuclearReactTypes::FNR => 700_000_000.0,
            NuclearReactTypes::HTGR => 200_000_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DieselGeneratorType {
    Home,
    Industrial,
}
//https://lifetide.co.za
impl DieselGeneratorType {
    pub fn value(&self) -> f64 {
        match self {
            DieselGeneratorType::Home => 5_000.0,
            DieselGeneratorType::Industrial => 100_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum PetrolGeneratorType {
    Home,
    Industrial,
}

//https://lifetide.co.za
impl PetrolGeneratorType {
    pub fn value(&self) -> f64 {
        match self {
            PetrolGeneratorType::Home => 6_500.0,
            PetrolGeneratorType::Industrial => 23_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CoalGeneratorType {
    Small,
    Medium,
    Large,
}

//https://satisfactory.fandom.com/wiki/Coal_Generator
impl CoalGeneratorType {
    pub fn value(&self) -> f64 {
        match self {
            CoalGeneratorType::Small => 63_000_000.0,
            CoalGeneratorType::Medium => 2_520_000.0,
            CoalGeneratorType::Large => 5_040_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum HydraulicTurbineType {
    Small,
    Medium,
    Large,
}

//https://www.micro-hydro-power.com/A-Guide-to-Hydro-Power.htm
//https://www.hydropower.org/case-study/worlds-largest-turbine
impl HydraulicTurbineType {
    pub fn value(&self) -> f64 {
        match self {
            HydraulicTurbineType::Small => 5_000_000.0,
            HydraulicTurbineType::Medium => 200_000_000.0,
            HydraulicTurbineType::Large => 1_000_000_000.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum GeneratorCurveType {
    SolarPanel(SolarPanelType),
    WindTurbine(WindTurbineType),
    NuclearReactor(NuclearReactTypes),
    DieselGenerator(DieselGeneratorType),
    PetrolGenerator(PetrolGeneratorType),
    CoalGenerator(CoalGeneratorType),
    HydraulicTurbine(HydraulicTurbineType),
}

impl GeneratorCurveType {
    fn value(&self) -> f64 {
        match self {
            GeneratorCurveType::SolarPanel(sort) => sort.value(),
            GeneratorCurveType::WindTurbine(sort) => sort.value(),
            GeneratorCurveType::NuclearReactor(sort) => sort.value(),
            GeneratorCurveType::DieselGenerator(sort) => sort.value(),
            GeneratorCurveType::PetrolGenerator(sort) => sort.value(),
            GeneratorCurveType::CoalGenerator(sort) => sort.value(),
            GeneratorCurveType::HydraulicTurbine(sort) => sort.value(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GeneratorCurve {
    generator_type: GeneratorCurveType,
    on_periods: Vec<Period>,
}

impl Curve for GeneratorCurve {
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
            self.generator_type.value()
        } else {
            0.0
        }
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let mut total = 0.0;
        for period in self.on_periods.iter() {
            total += period.duration() * self.generator_type.value();
        }
        total
    }
}

impl GeneratorCurve {
    pub fn add_period(&mut self, period: Period) {
        self.on_periods.push(period)
    }
}
