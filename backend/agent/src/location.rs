use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

impl Location {
    pub fn new() -> Location {
        let mut rng = rand::thread_rng();
        let latitude: f32 = rng.gen_range(27.985..28.104);
        let longitude: f32 = -rng.gen_range(26.148..26.629);

        return Location {
            longitude,
            latitude,
        };
    }
}
