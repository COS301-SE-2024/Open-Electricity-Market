

pub struct Resistnace(f32);

pub trait Consumer {
    fn get_impedance(&self) -> Resistnace;
}

