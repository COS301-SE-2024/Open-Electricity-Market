use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Instant,
};

use crate::agent_manager::producer::Voltage;

use self::{
    consumer::Consumer,
    producer::{
        ideal_producer::IdealProducer, Producer, ProducerBasics, ProducerManagerMessage,
        ProducerType,
    },
};

pub mod consumer;
pub mod producer;

#[derive(Clone)]
struct ProducerData {
    name: String,
    producer_type: ProducerType,
}

impl ProducerData {
    fn new(name: String, producer_type: ProducerType) -> ProducerData {
        return ProducerData {
            name,
            producer_type,
        };
    }
}

pub struct ProducerConroller {
    data: ProducerData,
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
    fn add_new_producer(&mut self, data: ProducerData) {
        let (tx, rx) = mpsc::channel();
        self.producers
            .push(Box::new(ProducerConroller { data :data.clone(), socket: rx }));

        thread::spawn(move || {
            let mut producer_basics = ProducerBasics::create(tx);
            let producer: Box<dyn Producer> = data.producer_type.actualise();

            let mut start = Instant::now();
            let mut elasped_time;
            let mut accumlited_time = 0.0;
            loop {
                let duration = start.elapsed();
                elasped_time = duration.as_secs_f32();
                start = Instant::now();
                accumlited_time += elasped_time;
                if producer.update(elasped_time, accumlited_time, &mut producer_basics) {
                    break;
                }
            }
        });
    }

    fn get_new_producers(&self) -> Vec<ProducerData> {
        let from_database = vec![
            ProducerData::new(String::from("a"), ProducerType::Ideal(Voltage(15.0))),
            ProducerData::new(String::from("b"), ProducerType::Ideal(Voltage(100.0))),
            ProducerData::new(String::from("c"), ProducerType::Ideal(Voltage(215.0))),
        ];
        let mut output = vec![];
        for producer_data in from_database {
            let mut contains = false;
            for producer_controller in self.producers.iter() {
                if producer_data.name == producer_controller.data.name {
                    contains = true;
                    break;
                }
            }

            if !contains {
                output.push(producer_data);
            }
        }
        return output;
    }

    fn check_producers(&mut self) {
        let mut to_delete = vec![];
        for producer_conroller in self.producers.iter() {
            match producer_conroller.socket.try_recv() {
                Ok(msg) => match msg {
                    ProducerManagerMessage::Stop => {
                        to_delete.push(producer_conroller.data.name.clone());
                    }
                },
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => {
                        continue;
                    }
                    mpsc::TryRecvError::Disconnected => {
                        to_delete.push(producer_conroller.data.name.clone())
                    }
                },
            }
        }
        for name in to_delete {
            self.producers.retain(|producer| producer.data.name != name)
        }
    }

    pub fn poll_database(&mut self) {
        loop {
            let producers = self.get_new_producers();
            for producer in producers {
                self.add_new_producer(producer);
            }

            //Check for messages from producer
            self.check_producers();
        }
    }
}
