use serde::Deserialize;

use crate::curve::Curve;

pub mod consumption_curve;

#[derive(Deserialize, Clone, Copy)]
pub struct SmartMeterDetail {
    pub circuit: u32,
    pub consumer: u32,
}

impl SmartMeterDetail {
    pub fn new() -> SmartMeterDetail {
        return SmartMeterDetail {
            circuit: 0,
            consumer: 0,
        };
    }
}
pub enum SmartMeter {
    Acctive(ActiveSmartMeterCore),
    InActtive,
}

impl SmartMeter {
    pub fn new_acctive(consumption_curve: Box<dyn Curve>) -> SmartMeter {
        return SmartMeter::Acctive(ActiveSmartMeterCore {
            grid_detail: SmartMeterDetail::new(),
            consumption_curve,
        });
    }
}

pub struct ActiveSmartMeterCore {
    pub grid_detail: SmartMeterDetail,
    pub consumption_curve: Box<dyn Curve>,
}
