pub mod ideal_producer;

use std::{io::Write, net::TcpStream, sync::mpsc::Sender};

use serde::Serialize;

use self::ideal_producer::IdealProducer;

#[derive(Clone, Copy)]
pub struct Voltage(pub f32);

#[derive(Clone)]
pub enum ProducerType {
    Ideal(Voltage),
}

impl ProducerType {
    pub fn actualise(&self) -> Box<dyn Producer> {
        match self {
            ProducerType::Ideal(voltage) => {
                return Box::new(IdealProducer {
                    voltage: voltage.clone(),
                });
            }
        }
    }
}

#[derive(Debug)]
pub enum ProducerManagerMessage {
    Stop,
}

#[derive(Serialize)]
pub struct GridInterface {
    circuit: u32,
    generator: u32,
    voltage: f32,
}

pub struct ProducerBasics {
    manager_socket: Sender<ProducerManagerMessage>,
    grid_socket: TcpStream,
    count: f32,
}
impl ProducerBasics {
    pub fn create(tx: Sender<ProducerManagerMessage>) -> ProducerBasics {
        return ProducerBasics {
            manager_socket: tx,
            grid_socket: TcpStream::connect("127.0.0.1:55555").unwrap(),
            count: 0.0,
        };
    }
}

pub trait Producer {
    fn get_voltage(&self, elasped_time: f32, accumlited_time: f32) -> Voltage;
    fn get_units_sold(&self) -> f32 {
        return 10.0;
    }
    fn send_message_manager(
        &self,
        message: ProducerManagerMessage,
        socket: Sender<ProducerManagerMessage>,
    ) {
        let _ = socket.send(message);
    }
    fn update_units_sold(&self, _voltage: Voltage) {}
    fn sync_grid(&self, voltage: Voltage, socket: &mut TcpStream) {
        let grid_interface = GridInterface {
            circuit: 0,
            generator: 0,
            voltage: voltage.0,
        };
        let mut json = serde_json::to_string(&grid_interface).unwrap();
        json.push_str("\r\n");
        let n = socket.write(json.as_bytes()).unwrap();
        println!("Wrote {n}");
    }
    fn update(
        &self,
        elasped_time: f32,
        accumlited_time: f32,
        producer_basics: &mut ProducerBasics,
    ) -> bool {
        // Get voltage
        let voltage = self.get_voltage(elasped_time, accumlited_time);

        //Check Database to see if I have sold units
        let units_sold = self.get_units_sold();
        //Make Desision
        if units_sold == 0.0 {
            //Tell manager and stop
            self.send_message_manager(
                ProducerManagerMessage::Stop,
                producer_basics.manager_socket.clone(),
            );
            return true;
        } else {
            //Upadate units sold and tell database
            self.update_units_sold(voltage);
        }

        //Every 5 mintues tell grid about my voltages
        if accumlited_time > 60.0 * producer_basics.count {
            producer_basics.count += 1.0;
            println!("a");
            self.sync_grid(voltage, &mut producer_basics.grid_socket);
            println!("b");
        }
        return false;
    }
}