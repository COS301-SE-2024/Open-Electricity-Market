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


struct CummutivelyCurve {
    curves: Vec<Box<dyn Curve>>
}

impl CummutivelyCurve {
    pub fn new() -> CummutivelyCurve {
        return CummutivelyCurve{ curves: vec![] };
    }
    pub fn addCurve(&mut self,curve :Box<dyn Curve>) {
        self.curves.push(curve);
    }
}

impl Curve for CummutivelyCurve {
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
}
