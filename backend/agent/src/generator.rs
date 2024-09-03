use serde::{Deserialize, Serialize};

use super::curve::Curve;

pub mod production_curve;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GeneratorDetail {
    pub circuit: u32,
    pub generator: u32,
}

impl Default for GeneratorDetail {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorDetail {
    pub fn new() -> GeneratorDetail {
        GeneratorDetail {
            circuit: 0,
            generator: 0,
        }
    }
}
#[derive(Serialize)]
pub enum Generator {
    Acctive(AcctiveGeneratorCore),
    InAcctive,
}

impl Generator {
    pub fn new_acctive(production_curve: Box<dyn Curve + Send + Sync>) -> Generator {
        Generator::Acctive(AcctiveGeneratorCore {
            grid_detail: GeneratorDetail::new(),
            production_curve,
        })
    }
}

#[derive(Serialize)]
pub struct AcctiveGeneratorCore {
    pub grid_detail: GeneratorDetail,
    pub production_curve: Box<dyn Curve + Send + Sync>,
}
