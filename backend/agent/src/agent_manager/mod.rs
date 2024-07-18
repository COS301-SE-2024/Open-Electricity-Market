use self::{consumer::Consumer, producer::Producer};

pub mod consumer;
pub mod producer;

struct AgentManager {
    consumers : Vec<Box<dyn Consumer>>,
    producer : Vec<Box<dyn Producer>>
}
