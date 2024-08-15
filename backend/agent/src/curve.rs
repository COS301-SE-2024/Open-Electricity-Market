pub trait Curve {
    fn sample(&mut self, time: f64) -> f64;
    fn total_in_24_hour(&mut self) -> f64;
}

pub struct SineCurve;

impl SineCurve {
    pub fn new() -> SineCurve {
        return SineCurve;
    }
}

impl Curve for SineCurve {
    fn sample(&mut self, time: f64) -> f64 {
        return f64::abs(f64::sin(time));
    }

    fn total_in_24_hour(&mut self) -> f64 {
        return 86.0;
    }
}
