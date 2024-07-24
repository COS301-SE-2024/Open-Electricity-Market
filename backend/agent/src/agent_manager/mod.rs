use std::{
    fmt::write, sync::mpsc::{self, Receiver}, thread, time::Instant
};

use crate::agent_manager::producer::Voltage;

use self::{
    consumer::Consumer,
    producer::{ideal_producer::IdealProducer, Producer, ProducerBasics, ProducerManagerMessage},
};

pub mod consumer;
pub mod producer;

pub struct ProducerConroller {
    name: String,
    socket: Receiver<ProducerManagerMessage>,
}

pub struct ConsumerController {
    consumer: dyn Consumer,
}

pub struct AgentManager {
    pub(crate) consumers: Vec<Box<ConsumerController>>,
    pub(crate) producers: Vec<Box<ProducerConroller>>,
}

impl AgentManager {
    fn add_new_producer(&mut self, name: String) {
        let (tx, rx) = mpsc::channel();
        self.producers.push(Box::new(ProducerConroller {
            name,
            socket: rx,
        }));

        

        thread::spawn(move || {
            let mut producer_basics = ProducerBasics::create(tx) ;
            let producer : Box<dyn Producer> = Box::new(IdealProducer { voltage: Voltage(0.0) });

            let mut start = Instant::now();
            let mut  elasped_time;
            let mut accumlited_time = 0.0;
            loop {
               let duration = start.elapsed();
               elasped_time = duration.as_secs_f32();               
               start = Instant::now();
               accumlited_time += elasped_time;
               producer.update(elasped_time, accumlited_time,&mut producer_basics);
            }
        });
    }

    fn get_new_producers(&self) -> Vec<String> {
        todo!();
    }

    pub fn poll_database(&mut self) {
        loop {
            let producers = self.get_new_producers();
            for producer in producers {
                self.add_new_producer(producer);
            }
        }
    }
}
