use std::sync::atomic::{AtomicU64, Ordering};

pub trait GridElement : Sync + Send {
    fn produce(&self,amps :u64);
    fn consume(& self,amps :u64) -> u64;
    fn get_avg_distribution_line_voltage(& self) -> f32;
}

pub trait PowerLine {
    fn get_current_voltages(& self) -> f32;
}

pub trait Transformer {
    fn step(proportion: f32) -> f32;
}

pub(crate) enum GridPiece {
    Object(Box<dyn GridElement>),
    Nil
}

pub struct DistributionLine {
    pub resistance : f32,
    pub amps : AtomicU64,
    pub to : GridPiece,
}

impl GridElement for DistributionLine{
    fn produce(& self,amps :u64){
         let old = self.amps.fetch_add(amps,Ordering::Relaxed);
    }
    fn consume(& self,amps :u64) -> u64{
       let current = self.amps.load(Ordering::Relaxed);
       if current > amps {
           self.amps.fetch_sub(amps,Ordering::Relaxed);
       }
        return current;
    }

    fn get_avg_distribution_line_voltage(&self) -> f32 {
        match &self.to {
            GridPiece::Object(to) => {
               return self.get_current_voltages() + to.as_ref().get_avg_distribution_line_voltage()
            }
            GridPiece::Nil => {
                return  self.get_current_voltages()
            }
        }
    }
}

impl PowerLine for DistributionLine {
    fn get_current_voltages(& self) -> f32 {
        let amps = self.amps.load(Ordering::Relaxed);
        return amps as f32 * self.resistance;
    }
}

pub struct Grid {
    pub(crate) grid : Box<dyn GridElement>,
}

impl Grid{
    pub fn generate(&mut self,amp :u64) {
        self.grid.produce(amp);
    }

    pub fn consume(&mut self,amp :u64) {
        self.grid.consume(amp);
    }

    pub fn get_avg_distribution_line_voltage() -> f32 {
        0.0
    }
}