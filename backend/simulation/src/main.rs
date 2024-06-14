#[macro_use]
extern crate rocket;

use std::cell::RefCell;
use std::cmp::max;
use std::sync::{Arc, Mutex};
use rocket::State;
use std::sync::atomic::AtomicU64;
use std::thread::current;
use rocket::serde::json::json;
use rocket::yansi::Paint;
use std::time::{Duration, Instant};


trait ToJson {
    fn to_json(&self) -> String;
}

struct Generator {
    id : u32,
    voltage: Voltage,
    max_voltage:f32,
    frequency :f32,
    transmission_line: u32,
}

impl ToJson for Generator {
    fn to_json(&self) -> String{
        json!({ "ID" : self.id,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            },
            "Max Voltage" : self.max_voltage,
            "Frequency" : self.frequency,
            "Connected Transmission Line" : self.transmission_line
        }).to_string()
    }
}


struct Consumer {
    id : u32,
    resistance: Resistance,
    transmission_line: u32,
    voltage: Voltage
}

impl ToJson for Consumer {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
            "Resistance" : self.resistance.0,
            "Connected Transmission Line" : self.transmission_line,
             "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            }
        }).to_string()
    }
}

struct TransmissionLine {
    id : u32,
    resistance: Resistance,
    impedance: Resistance,
    voltage: Voltage
}

impl ToJson for TransmissionLine {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
            "Resistance" : self.resistance.0,
            "Impedance" : self.impedance.0,
            "Voltage" : {
                "Phase 1" : self.voltage.0,
                "Phase 2" : self.voltage.1,
                "Phase 3" : self.voltage.2,
            }
        }).to_string()
    }
}

struct Transformer {
    id : u32,
    ratio : f32,
    primary : u32,
    secondary : u32,
}

impl ToJson for Transformer {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
                "Ratio" : self.ratio,
                "Primary Transmission Line" : self.primary,
                "Secondary Transmission Line" : self.secondary
        }).to_string()
    }
}

struct Resistance(f32);
struct Voltage(f32,f32,f32);
struct Grid {
    consumers : Vec<Consumer>,
    transmission_lines: Vec<TransmissionLine>,
    generators : Vec<Generator>,
    transformers: Vec<Transformer>,
    started : bool
}

impl Grid {
    fn update_impedance(&mut self) {
        for line in self.transmission_lines.iter_mut() {
            let index =self.consumers.iter().position( |c| c.transmission_line == line.id);
            match index {
                None => {line.impedance = Resistance(line.resistance.0)}
                Some(i) => {line.impedance = Resistance(line.resistance.0+self.consumers[i].resistance.0)}
            }

        }
    }

    fn update_generator_voltages(&mut self, elapsed_time:f32){
        for gen in self.generators.iter_mut() {
            gen.voltage.0 = gen.max_voltage*f32::sin(gen.frequency*std::f32::consts::FRAC_2_PI*elapsed_time);
            gen.voltage.1 = gen.max_voltage*f32::sin(gen.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(std::f32::consts::FRAC_2_PI/3.0));
            gen.voltage.2 = gen.max_voltage*f32::sin(gen.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(2.0*std::f32::consts::FRAC_2_PI/3.0));
        }
    }

    fn sync_voltages(&mut self) {
        // Gens to lines
        for gen in self.generators.iter_mut() {
            let index = self.transmission_lines.iter().position( |l| l.id == gen.transmission_line);
            match index {
                None => {}
                Some(i) => {
                    self.transmission_lines[i].voltage.0 = gen.voltage.0;
                    self.transmission_lines[i].voltage.1 = gen.voltage.1;
                    self.transmission_lines[i].voltage.2 = gen.voltage.2;
                }
            }
        }
        // Steps
        for trans in self.transformers.iter() {
            let primary_index = self.transmission_lines.iter().position(|l| l.id == trans.primary);
            let secondary_index = self.transmission_lines.iter().position(|l| l.id == trans.secondary);
            match primary_index {
                None => {}
                Some(i) => {
                    match secondary_index {
                        None => {}
                        Some(j) => {
                            self.transmission_lines[j].voltage.0 = self.transmission_lines[i].voltage.0*trans.ratio;
                            self.transmission_lines[j].voltage.1 = self.transmission_lines[i].voltage.1*trans.ratio;
                            self.transmission_lines[j].voltage.2 = self.transmission_lines[i].voltage.2*trans.ratio;
                        }
                    }

                }
            }
        }

        // Lines to consumers
        for line in self.transmission_lines.iter_mut() {
            let index = self.consumers.iter().position( |c| c.transmission_line == line.id);
            match index {
                None => {}
                Some(i) => {
                    self.consumers[i].voltage.0 = line.voltage.0;
                    self.consumers[i].voltage.1 = line.voltage.1;
                    self.consumers[i].voltage.2 = line.voltage.2;
                }
            }
        }


    }
}

impl ToJson for Grid {
    fn to_json(&self) -> String {
        // consumers : Vec<Consumer>,
        // transmission_lines: Vec<TransmissionLine>,
        // generators : Vec<Generator>,
        // transformers: Vec<Transformer>,
        // started : bool
        let mut consumer_strings: Vec<String> = vec![];
        for consumer in self.consumers.iter() {
           consumer_strings.push(consumer.to_json());
        };

        let mut transmission_line_strings: Vec<String> = vec![];
        for lines in self.transmission_lines.iter() {
            transmission_line_strings.push(lines.to_json());
        };

        let mut generator_strings : Vec<String> = vec![];
        for gen in self.generators.iter() {
            generator_strings.push(gen.to_json());
        }

        let mut transformer_strings : Vec<String> = vec![];
        for trans in self.transformers.iter() {
            transformer_strings.push(trans.to_json());
        }

        json!({"Consumers" : consumer_strings,
            "Transmission Lines" : transmission_line_strings,
            "Generators" : generator_strings,
            "Transformers" : transformer_strings,
            "Started" : self.started
        }).to_string()
    }
}

#[get("/")]
fn index() -> String {
    "Yay".to_string()
}

#[get("/produce/<amount>")]
fn produce(amount: u64) -> String {
    "produce".to_string()
}

#[get("/consume/<amount>")]
fn consume(amount: u64) -> String {
    format!("Consume a")
}

#[get("/info")]
fn info(grid: &State<Arc<Mutex<Grid>>>) ->String {
    let g = grid.lock().unwrap();
    g.to_json()
}

#[get("/start")]
fn start(grid: &State<Arc<Mutex<Grid>>>) -> String {
    let mut g = grid.lock().unwrap();
    if !g.started {
        g.started = true;
        let clone = grid.inner().clone();
        tokio::spawn(async move{
            let mut start = Instant::now();
            let mut elapsed_time = 0.0;
            loop {
                let duration = start.elapsed();
                elapsed_time += duration.as_secs_f32();
                let mut grid = clone.lock().unwrap();
                grid.update_impedance();
                grid.update_generator_voltages(elapsed_time);
                grid.sync_voltages();
                start = Instant::now();
            }
        });
        "Grid Started".to_string()
    } else {
        "Grid Already Running".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, produce, consume,start,info])
        .manage(Arc::new(Mutex::new(Grid {
            consumers: vec![
                Consumer {id: 1,resistance: Resistance(1000.0),transmission_line: 1, voltage: Voltage(0.0,0.0,0.0)}
            ],
            transmission_lines: vec![
                TransmissionLine {id:0,resistance: Resistance(50.0) , impedance: Resistance(0.0), voltage: Voltage(0.0, 0.0, 0.0)},
                TransmissionLine {id:1,resistance: Resistance(50.0) , impedance: Resistance(0.0), voltage: Voltage(0.0, 0.0, 0.0)}

            ],
            generators: vec![
                Generator {id:0,voltage: Voltage(0.0,0.0,0.0), max_voltage: 240.0, frequency: 50.0, transmission_line: 0 }

            ],
            transformers: vec![
                Transformer {id:0,ratio:0.5,primary:0,secondary:1}
            ],
            started: false,
        })))
}
