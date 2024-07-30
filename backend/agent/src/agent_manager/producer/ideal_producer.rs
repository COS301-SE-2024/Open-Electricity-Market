use super::{Producer, Voltage};

pub struct IdealProducer {
    pub voltage: Voltage,
}

impl Producer for IdealProducer {
    fn get_voltage(&self, elasped_time: f32, accumlited_time: f32) -> Voltage {
        return self.voltage;
    }
    // add code here
}
