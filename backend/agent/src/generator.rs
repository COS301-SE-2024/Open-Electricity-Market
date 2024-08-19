use serde::Deserialize;

use super::curve::Curve;

pub mod production_curve;

#[derive(Deserialize, Clone, Copy)]
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

pub struct AcctiveGeneratorCore {
    pub grid_detail: GeneratorDetail,
    pub production_curve: Box<dyn Curve + Send + Sync>,
}
