pub trait Curve {
    fn sample(&mut self, time: f64) -> f64;
    fn total_in_24_hour(&mut self) -> f64;
    fn add_curve(&mut self, _curve: Box<dyn Curve + Send + Sync>) {
        //DO nothing
    }
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

pub struct CummutiveCurve {
    curves: Vec<Box<dyn Curve + Send + Sync>>,
}

impl CummutiveCurve {
    pub fn new() -> CummutiveCurve {
        return CummutiveCurve { curves: vec![] };
    }
}

impl Curve for CummutiveCurve {
    fn sample(&mut self, time: f64) -> f64 {
        let mut total = 0.0;
        for curve in self.curves.iter_mut() {
            total += curve.sample(time);
        }
        return total;
    }

    fn total_in_24_hour(&mut self) -> f64 {
        let mut total = 0.0;
        for curve in self.curves.iter_mut() {
            total += curve.total_in_24_hour();
        }
        return total;
    }
    fn add_curve(&mut self, curve: Box<dyn Curve + Send + Sync>) {
        self.curves.push(curve);
    }
}
