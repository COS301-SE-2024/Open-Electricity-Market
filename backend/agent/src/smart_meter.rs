use serde::Deserialize;

use crate::curve::Curve;

pub mod consumption_curve;

#[derive(Deserialize, Clone, Copy)]
pub struct SmartMeterDetail {
    pub circuit: u32,
    pub consumer: u32,
}

impl Default for SmartMeterDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartMeterDetail {
    pub fn new() -> SmartMeterDetail {
        SmartMeterDetail {
            circuit: 0,
            consumer: 0,
        }
    }
}
pub enum SmartMeter {
    Acctive(ActiveSmartMeterCore),
    InActtive,
}

impl SmartMeter {
    pub fn new_acctive(consumption_curve: Box<dyn Curve + Send + Sync>) -> SmartMeter {
        SmartMeter::Acctive(ActiveSmartMeterCore {
            grid_detail: SmartMeterDetail::new(),
            consumption_curve,
        })
    }
}

pub struct ActiveSmartMeterCore {
    pub grid_detail: SmartMeterDetail,
    pub consumption_curve: Box<dyn Curve + Send + Sync>,
}