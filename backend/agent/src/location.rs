use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Location {
        let mut rng = rand::thread_rng();
        let latitude: f32 = rng.gen_range(28.092..28.365);
        let longitude: f32 = -rng.gen_range(25.624..25.971);

        Location {
            longitude,
            latitude,
        }
    }
}
