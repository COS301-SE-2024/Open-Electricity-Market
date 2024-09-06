use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

use crate::{generator::production_curve::GeneratorCurveType, period::Period};

pub trait Curve: erased_serde::Serialize {
    fn sample(&mut self, time: f64) -> f64;
    fn total_in_24_hour(&mut self) -> f64;
    fn add_curve(&mut self, _curve: Box<dyn Curve + Send + Sync>) {
        //DO nothing
    }
    fn get_appliance_list_if_possible(&mut self) -> Vec<String> {
        return vec![];
    }
    fn get_generator_curve_if_possible(&mut self) -> Vec<(GeneratorCurveType, f64, Vec<Period>)> {
        return vec![];
    }
}

serialize_trait_object!(Curve);

#[derive(Serialize)]
pub struct SineCurve;

impl Default for SineCurve {
    fn default() -> Self {
        Self::new()
    }
}

impl SineCurve {
    pub fn new() -> SineCurve {
        SineCurve
    }
}

impl Curve for SineCurve {
    fn sample(&mut self, time: f64) -> f64 {
        f64::abs(f64::sin(time)) * 300.0
    }

    fn total_in_24_hour(&mut self) -> f64 {
        86.0
    }
}

#[derive(Serialize)]
pub struct CummutiveCurve {
    curves: Vec<Box<dyn Curve + Send + Sync>>,
}

impl Default for CummutiveCurve {
    fn default() -> Self {
        Self::new()
    }
}

impl CummutiveCurve {
    pub fn new() -> CummutiveCurve {
        CummutiveCurve { curves: vec![] }
    }
}

impl Curve for CummutiveCurve {
    fn sample(&mut self, time: f64) -> f64 {
        let mut total = 0.0;
        for curve in self.curves.iter_mut() {
            total += curve.sample(time);
        }
        total
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let mut total = 0.0;
        for curve in self.curves.iter_mut() {
            total += curve.total_in_24_hour();
        }
        total
    }
    fn add_curve(&mut self, curve: Box<dyn Curve + Send + Sync>) {
        self.curves.push(curve);
    }

    fn get_appliance_list_if_possible(&mut self) -> Vec<String> {
        let mut out = vec![];

        for curve in self.curves.iter_mut() {
            out.append(&mut curve.get_appliance_list_if_possible())
        }
        return out;
    }

    fn get_generator_curve_if_possible(&mut self) -> Vec<(GeneratorCurveType, f64, Vec<Period>)> {
        let mut out = vec![];

        for curve in self.curves.iter_mut() {
            out.append(&mut curve.get_generator_curve_if_possible())
        }
        return out;
    }
}
