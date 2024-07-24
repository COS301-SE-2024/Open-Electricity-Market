pub mod ideal_producer;

use std::{net::TcpStream, sync::mpsc::Sender};

#[derive(Clone, Copy)]
pub struct Voltage(pub f32);

#[derive(Debug)]
pub enum ProducerManagerMessage {
    Stop,
}

pub struct ProducerBasics {
    manager_socket: Sender<ProducerManagerMessage>,
    grid_socket: TcpStream,
}

impl ProducerBasics {
    pub fn create(tx : Sender<ProducerManagerMessage>) -> ProducerBasics {
        return  ProducerBasics {
            manager_socket : tx,
            grid_socket : TcpStream::connect("127.0.0.1:55555").unwrap()
        };
    }
}

pub trait Producer {
    fn get_voltage(&self, elasped_time: f32, accumlited_time: f32) -> Voltage;
    fn get_units_sold(&self) -> f32 {
        return 0.0;
    }
    fn send_message_manager(
        &self,
        message: ProducerManagerMessage,
        socket: Sender<ProducerManagerMessage>,
    ) {
    }
    fn update_units_sold(&self, voltage: Voltage) {}
    fn sync_grid(&self, voltage: Voltage) {}
    fn update(
        &self,
        elasped_time: f32,
        accumlited_time: f32,
        producer_basics :&mut ProducerBasics
    ) {
        // Get voltage
        let voltage = self.get_voltage(elasped_time, accumlited_time);

        //Check Database to see if I have sold units
        let units_sold = self.get_units_sold();
        //Make Desision
        if units_sold == 0.0 {
            //Tell manager and stop
            self.send_message_manager(ProducerManagerMessage::Stop, producer_basics.manager_socket.clone());
        } else {
            //Upadate units sold and tell database
            self.update_units_sold(voltage);
        }

        //Every 5 mintues tell grid about my voltages
        if accumlited_time / 60.0 <= 0.01 {
            self.sync_grid(voltage);
        }
    }
}
