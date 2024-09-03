use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Period {
    start: f64,
    end: f64,
}

impl Period {
    pub fn during(&self, time: f64) -> bool {
        self.start < time && self.end > time
    }
    pub fn duration(&self) -> f64 {
        self.end - self.start
    }
}
