use crate::{
    curve::{CummutiveCurve, Curve, SineCurve},
    generator::production_curve::{
        CoalGeneratorType, DieselGeneratorType, GeneratorCurve, GeneratorCurveType,
        HydraulicTurbineType, NuclearReactTypes, PetrolGeneratorType, SolarPanelType,
        WindTurbineType,
    },
    period::{self, Period},
    smart_meter::consumption_curve::{HomeAppliance, HomeApplianceType},
};
#[test]
pub fn solar_panel_type_home_value() {
    let value = SolarPanelType::Home.value();
    assert_eq!(value, 325.0);
}
#[test]
pub fn solar_panel_type_industrial_value() {
    let value = SolarPanelType::Industrial.value();
    assert_eq!(value, 375.0);
}
#[test]
pub fn wind_turbine_type_small_value() {
    let value = WindTurbineType::Small.value();
    assert_eq!(value, 700_000.0);
}
#[test]
pub fn wind_turbine_type_medium_value() {
    let value = WindTurbineType::Medium.value();
    assert_eq!(value, 2_000_000.0);
}
#[test]
pub fn wind_turbine_type_large_value() {
    let value = WindTurbineType::Large.value();
    assert_eq!(value, 3_200_000.0);
}
#[test]
pub fn nuclear_react_types_pwr_value() {
    let value = NuclearReactTypes::PWR.value();
    assert_eq!(value, 956_000_000.0);
}
#[test]
pub fn nuclear_react_types_bwr_value() {
    let value = NuclearReactTypes::BWR.value();
    assert_eq!(value, 1_015_000_000.0);
}
#[test]
pub fn nuclear_react_types_phwr_value() {
    let value = NuclearReactTypes::PHWR.value();
    assert_eq!(value, 520_000_000.0);
}
#[test]
pub fn nuclear_react_types_lwgr_value() {
    let value = NuclearReactTypes::LWGR.value();
    assert_eq!(value, 650_000_000.0);
}
#[test]
pub fn nuclear_react_types_agr_value() {
    let value = NuclearReactTypes::AGR.value();
    assert_eq!(value, 580_000_000.0);
}
#[test]
pub fn nuclear_react_types_fnr_value() {
    let value = NuclearReactTypes::FNR.value();
    assert_eq!(value, 700_000_000.0);
}
#[test]
pub fn nuclear_react_types_htgr_value() {
    let value = NuclearReactTypes::HTGR.value();
    assert_eq!(value, 200_000_000.0);
}
#[test]
pub fn diesel_generator_type_home_value() {
    let value = DieselGeneratorType::Home.value();
    assert_eq!(value, 5_000.0);
}
#[test]
pub fn diesel_generator_type_industrial_value() {
    let value = DieselGeneratorType::Industrial.value();
    assert_eq!(value, 100_000.0);
}
#[test]
pub fn petrol_generator_type_home_value() {
    let value = PetrolGeneratorType::Home.value();
    assert_eq!(value, 6_500.0);
}
#[test]
pub fn petrol_generator_type_industrial_value() {
    let value = PetrolGeneratorType::Industrial.value();
    assert_eq!(value, 23_000.0);
}
#[test]
pub fn coal_generator_type_small_value() {
    let value = CoalGeneratorType::Small.value();
    assert_eq!(value, 630_000.0);
}
#[test]
pub fn coal_generator_type_medium_value() {
    let value = CoalGeneratorType::Medium.value();
    assert_eq!(value, 2_520_000.0);
}
#[test]
pub fn coal_generator_type_large_value() {
    let value = CoalGeneratorType::Large.value();
    assert_eq!(value, 5_040_000.0);
}
#[test]
pub fn hydraulic_turbine_type_small_value() {
    let value = HydraulicTurbineType::Small.value();
    assert_eq!(value, 5_000_000.0);
}
#[test]
pub fn hydraulic_turbine_type_medium_value() {
    let value = HydraulicTurbineType::Medium.value();
    assert_eq!(value, 200_000_000.0);
}
#[test]
pub fn hydraulic_turbine_type_large_value() {
    let value = HydraulicTurbineType::Large.value();
    assert_eq!(value, 1_000_000_000.0);
}

#[test]
pub fn generator_curve_type_solar_panel_value() {
    let value = GeneratorCurveType::SolarPanel(SolarPanelType::Home).value();
    assert_eq!(value, 325.0);
    let value = GeneratorCurveType::SolarPanel(SolarPanelType::Industrial).value();
    assert_eq!(value, 375.0);
}
#[test]
pub fn generator_curve_type_wind_turbine_value() {
    let value = GeneratorCurveType::WindTurbine(WindTurbineType::Small).value();
    assert_eq!(value, 700_000.0);
    let value = GeneratorCurveType::WindTurbine(WindTurbineType::Medium).value();
    assert_eq!(value, 2_000_000.0);
    let value = GeneratorCurveType::WindTurbine(WindTurbineType::Large).value();
    assert_eq!(value, 3_200_000.0);
}
#[test]
pub fn generator_curve_type_nuclear_reactor_value() {
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::PWR).value();
    assert_eq!(value, 956_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::BWR).value();
    assert_eq!(value, 1_015_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::PHWR).value();
    assert_eq!(value, 520_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::LWGR).value();
    assert_eq!(value, 650_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::AGR).value();
    assert_eq!(value, 580_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::FNR).value();
    assert_eq!(value, 700_000_000.0,);
    let value = GeneratorCurveType::NuclearReactor(NuclearReactTypes::HTGR).value();
    assert_eq!(value, 200_000_000.0,);
}
#[test]
pub fn generator_curve_type_diesel_generator_value() {
    let value = GeneratorCurveType::DieselGenerator(DieselGeneratorType::Home).value();
    assert_eq!(value, 5_000.0);
    let value = GeneratorCurveType::DieselGenerator(DieselGeneratorType::Industrial).value();
    assert_eq!(value, 100_000.0);
}
#[test]
pub fn generator_curve_type_petrol_generator_value() {
    let value = GeneratorCurveType::PetrolGenerator(PetrolGeneratorType::Home).value();
    assert_eq!(value, 6_500.0);
    let value = GeneratorCurveType::PetrolGenerator(PetrolGeneratorType::Industrial).value();
    assert_eq!(value, 23_000.0);
}
#[test]
pub fn generator_curve_type_coal_generator_value() {
    let value = GeneratorCurveType::CoalGenerator(CoalGeneratorType::Small).value();
    assert_eq!(value, 630_000.0);
    let value = GeneratorCurveType::CoalGenerator(CoalGeneratorType::Medium).value();
    assert_eq!(value, 2_520_000.0);
    let value = GeneratorCurveType::CoalGenerator(CoalGeneratorType::Large).value();
    assert_eq!(value, 5_040_000.0);
}
#[test]
pub fn generator_curve_type_hydraulic_turbine_value() {
    let value = GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Small).value();
    assert_eq!(value, 5_000_000.0);
    let value = GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Medium).value();
    assert_eq!(value, 200_000_000.0);
    let value = GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Large).value();
    assert_eq!(value, 1_000_000_000.0);
}

#[test]
pub fn add_on_period() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    generator_curve.add_period(crate::period::Period {
        start: 0.0,
        end: 15.0,
    });
    assert_eq!(generator_curve.on_periods.len(), 1);
    generator_curve.add_period(crate::period::Period {
        start: 30.0,
        end: 45.0,
    });
    assert_eq!(generator_curve.on_periods.len(), 2);
}

#[test]
pub fn generator_curve_sample_during_on_period() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![crate::period::Period {
            start: 0.0,
            end: 15.0,
        }],
    };
    assert_eq!(generator_curve.sample(3.0), 325.0)
}

#[test]
pub fn generator_curve_sample_not_during_on_period() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![crate::period::Period {
            start: 0.0,
            end: 15.0,
        }],
    };
    assert_eq!(generator_curve.sample(18.0), 0.0)
}

#[test]
pub fn generator_curve_sample_with_no_periods() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    assert_eq!(generator_curve.sample(18.0), 0.0)
}

#[test]
pub fn generator_curve_total_in_24_hours_with_no_periods() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    assert_eq!(generator_curve.total_in_24_hour(), 0.0)
}

#[test]
pub fn generator_curve_total_in_24_hours_with_periods() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![
            crate::period::Period {
                start: 0.0,
                end: 15.0,
            },
            crate::period::Period {
                start: 30.0,
                end: 45.0,
            },
        ],
    };
    assert_eq!(generator_curve.total_in_24_hour(), 325.0 * 30.0)
}

#[test]
pub fn generator_curve_get_generator_curve_if_possible_with_no_periods() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    let (_, value, vec) = &generator_curve.get_generator_curve_if_possible()[0];
    let length = vec.len();
    assert_eq!(*value, 325.0);
    assert_eq!(length, 0)
}

#[test]
pub fn generator_curve_get_generator_curve_if_possible_with_periods() {
    let mut generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![
            crate::period::Period {
                start: 0.0,
                end: 15.0,
            },
            crate::period::Period {
                start: 30.0,
                end: 45.0,
            },
        ],
    };
    let (_, value, vec) = &generator_curve.get_generator_curve_if_possible()[0];
    let length = vec.len();
    assert_eq!(*value, 325.0);
    assert_eq!(length, 2)
}

#[test]
pub fn home_appliance_type_washing_machine_to_string() {
    let value = HomeApplianceType::WashingMachine.to_string();
    assert_eq!(value, String::from("washing_machine"));
}
#[test]
pub fn home_appliance_type_router_to_string() {
    let value = HomeApplianceType::Router.to_string();
    assert_eq!(value, String::from("router"));
}
#[test]
pub fn home_appliance_type_vacuum_to_string() {
    let value = HomeApplianceType::Vacuum.to_string();
    assert_eq!(value, String::from("vacuum"));
}
#[test]
pub fn home_appliance_type_dishwasher_to_string() {
    let value = HomeApplianceType::Dishwasher.to_string();
    assert_eq!(value, String::from("dish_washer"));
}
#[test]
pub fn home_appliance_type_boiler_to_string() {
    let value = HomeApplianceType::Boiler.to_string();
    assert_eq!(value, String::from("boiler"));
}
#[test]
pub fn home_appliance_type_hair_purifier_to_string() {
    let value = HomeApplianceType::HairPurifier.to_string();
    assert_eq!(value, String::from("hair_purifier"));
}
#[test]
pub fn home_appliance_type_sound_system_to_string() {
    let value = HomeApplianceType::SoundSystem.to_string();
    assert_eq!(value, String::from("sound_system"));
}
#[test]
pub fn home_appliance_type_printer3d_to_string() {
    let value = HomeApplianceType::Printer3d.to_string();
    assert_eq!(value, String::from("printer_3D"));
}
#[test]
pub fn home_appliance_type_coffee_machine_to_string() {
    let value = HomeApplianceType::CoffeeMachine.to_string();
    assert_eq!(value, String::from("coffee"));
}
#[test]
pub fn home_appliance_type_phone_charger_to_string() {
    let value = HomeApplianceType::PhoneCharger.to_string();
    assert_eq!(value, String::from("phone_charger"));
}
#[test]
pub fn home_appliance_type_fridge_to_string() {
    let value = HomeApplianceType::Fridge.to_string();
    assert_eq!(value, String::from("fridge"));
}
#[test]
pub fn home_appliance_type_radiator_to_string() {
    let value = HomeApplianceType::Radiator.to_string();
    assert_eq!(value, String::from("radiator"));
}
#[test]
pub fn home_appliance_type_dehumidifier_to_string() {
    let value = HomeApplianceType::Dehumidifier.to_string();
    assert_eq!(value, String::from("dehumidifier"));
}
#[test]
pub fn home_appliance_type_micro_wave_oven_to_string() {
    let value = HomeApplianceType::MicroWaveOven.to_string();
    assert_eq!(value, String::from("micro_wave_oven"));
}
#[test]
pub fn home_appliance_type_laptop_to_string() {
    let value = HomeApplianceType::Laptop.to_string();
    assert_eq!(value, String::from("laptop"));
}
#[test]
pub fn home_appliance_type_tv_to_string() {
    let value = HomeApplianceType::Tv.to_string();
    assert_eq!(value, String::from("tv"));
}
#[test]
pub fn home_appliance_type_screen_to_string() {
    let value = HomeApplianceType::Screen.to_string();
    assert_eq!(value, String::from("screen"));
}
#[test]
pub fn home_appliance_type_fan_to_string() {
    let value = HomeApplianceType::Fan.to_string();
    assert_eq!(value, String::from("fan"));
}
#[test]
pub fn home_appliance_type_air_conditioner_to_string() {
    let value = HomeApplianceType::AirConditioner.to_string();
    assert_eq!(value, String::from("air_conditioner"));
}
#[test]
pub fn home_appliance_type_computer_to_string() {
    let value = HomeApplianceType::Computer.to_string();
    assert_eq!(value, String::from("computer"));
}
#[test]
pub fn home_appliance_type_printer_to_string() {
    let value = HomeApplianceType::Printer.to_string();
    assert_eq!(value, String::from("printer"));
}
#[test]
pub fn home_appliance_type_dryer_to_string() {
    let value = HomeApplianceType::Dryer.to_string();
    assert_eq!(value, String::from("dryer"));
}
#[test]
pub fn home_appliance_type_freezer_to_string() {
    let value = HomeApplianceType::Freezer.to_string();
    assert_eq!(value, String::from("freezer"));
}
#[test]
pub fn home_appliance_get_appliance_list_if_possible() {
    let mut home_appliance = HomeAppliance {
        appliance_type: HomeApplianceType::WashingMachine,
    };
    let list = home_appliance.get_appliance_list_if_possible();
    assert_eq!(list[0], String::from("washing_machine"))
}
#[test]
pub fn sine_curve_sample() {
    let mut sine_curve = SineCurve {};
    let value = sine_curve.sample(0.0);
    assert_eq!(value, 300.0);
    let value = sine_curve.sample(512.0);
    assert!(value < 600.0);
}
#[test]
pub fn sine_curve_total_in_24_hour() {
    let mut sine_curve = SineCurve {};
    let value = sine_curve.total_in_24_hour();
    assert_eq!(value, 86.0);
}

#[test]
pub fn cummitive_curve_get_aplliance_list_if_possible_0() {
    let mut cummitive_curve = CummutiveCurve { curves: vec![] };
    let list = cummitive_curve.get_appliance_list_if_possible();
    assert_eq!(list, Vec::<String>::new())
}

#[test]
pub fn cummitive_curve_get_aplliance_list_if_possible_1() {
    let home_appliance = HomeAppliance {
        appliance_type: HomeApplianceType::WashingMachine,
    };
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(home_appliance)],
    };
    let list = cummitive_curve.get_appliance_list_if_possible();
    assert_eq!(list, vec![String::from("washing_machine")])
}

#[test]
pub fn cummitive_curve_get_aplliance_list_if_possible_more_than_one() {
    let home_appliance = HomeAppliance {
        appliance_type: HomeApplianceType::WashingMachine,
    };
    let home_appliance2 = HomeAppliance {
        appliance_type: HomeApplianceType::Boiler,
    };
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(home_appliance), Box::new(home_appliance2)],
    };
    let list = cummitive_curve.get_appliance_list_if_possible();
    assert_eq!(
        list,
        vec![String::from("washing_machine"), String::from("boiler")]
    )
}

#[test]
pub fn cummitive_curve_get_generator_curve_if_possible_0() {
    let mut cummitive_curve = CummutiveCurve { curves: vec![] };
    let result = cummitive_curve.get_generator_curve_if_possible();
    assert_eq!(result.len(), 0)
}

#[test]
pub fn cummitive_curve_get_generator_curve_if_possible_1() {
    let generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(generator_curve)],
    };
    let result = cummitive_curve.get_generator_curve_if_possible();
    assert_eq!(result.len(), 1)
}

#[test]
pub fn cummitive_curve_get_generator_curve_if_possible_more_than_one() {
    let generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    let generator_curve2 = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![],
    };
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(generator_curve), Box::new(generator_curve2)],
    };
    let result = cummitive_curve.get_generator_curve_if_possible();
    assert_eq!(result.len(), 2)
}

#[test]
pub fn cummitive_curve_sample_multiple() {
    let generator_curve = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![crate::period::Period {
            start: 0.0,
            end: 15.0,
        }],
    };
    let generator_curve2 = GeneratorCurve {
        generator_type: GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        on_periods: vec![crate::period::Period {
            start: 0.0,
            end: 30.0,
        }],
    };
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(generator_curve), Box::new(generator_curve2)],
    };
    assert_eq!(cummitive_curve.sample(0.0), 325.0 * 2.0);
    assert_eq!(cummitive_curve.sample(20.0), 325.0);
    assert_eq!(cummitive_curve.sample(40.0), 0.0);
}

#[test]
pub fn cummitive_curve_sample_none() {
    let mut cummitive_curve = CummutiveCurve { curves: vec![] };
    assert_eq!(cummitive_curve.sample(0.0), 0.0);
    assert_eq!(cummitive_curve.sample(20.0), 0.0);
}

#[test]
pub fn cummitive_curve_total_in_24_hours_none() {
    let mut cummitive_curve = CummutiveCurve { curves: vec![] };
    assert_eq!(cummitive_curve.total_in_24_hour(), 0.0);
}

#[test]
pub fn cummitive_curve_total_in_24_hours_multiple() {
    let mut cummitive_curve = CummutiveCurve {
        curves: vec![Box::new(SineCurve), Box::new(SineCurve)],
    };
    assert_eq!(cummitive_curve.total_in_24_hour(), 86.0 * 2.0);
}

#[test]
pub fn cummitive_curve_add_curve() {
    let mut cummutive_curve = CummutiveCurve { curves: vec![] };
    assert_eq!(cummutive_curve.curves.len(), 0);
    cummutive_curve.add_curve(Box::new(SineCurve));
    assert_eq!(cummutive_curve.curves.len(), 1);
    cummutive_curve.add_curve(Box::new(SineCurve));
    assert_eq!(cummutive_curve.curves.len(), 2);
    cummutive_curve.add_curve(Box::new(SineCurve));
    assert_eq!(cummutive_curve.curves.len(), 3)
}

#[test]
pub fn period_during() {
    let period = Period {
        start: 0.0,
        end: 15.0,
    };
    assert_eq!(period.during(0.0), true);
    assert_eq!(period.during(16.0), false)
}

#[test]
pub fn period_duration_non_0() {
    let period = Period {
        start: 0.0,
        end: 15.0,
    };
    assert_eq!(period.duration(), 15.0)
}

#[test]
pub fn period_duration_0() {
    let period = Period {
        start: 0.0,
        end: 0.0,
    };
    assert_eq!(period.duration(), 0.0)
}
