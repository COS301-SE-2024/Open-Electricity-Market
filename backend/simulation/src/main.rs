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


struct Location<'a> {
  equipment: Vec<GridEquipment<'a>>,

}

trait ACEquipment {
    fn update(&mut self, elapsed_time: f32);
    fn get_json(&self) -> String;
}

enum GridEquipment<'a> {
    Consumer(Consumer<'a>),
    ACSwitches(ACSwitches),
    Transformers(Transformers<'a>),
    Generators(Generators<'a>)
}

struct Consumer<'a> {
    supplier:  &'a Arc<Mutex<TransmissionLines>>,
    load : f32
}

impl ACEquipment for Consumer<'_> {
    fn update(&mut self, elapsed_time: f32) {
        let mut supplier_voltage =self.supplier.lock().unwrap();

        if supplier_voltage.voltage.phase1 > supplier_voltage.voltage.phase2 && supplier_voltage.voltage.phase1 > supplier_voltage.voltage.phase3{
            self.load = supplier_voltage.voltage.phase1;
        } else if supplier_voltage.voltage.phase2 > supplier_voltage.voltage.phase3 {
            self.load = supplier_voltage.voltage.phase2;
        } else {
            self.load = supplier_voltage.voltage.phase3;
        }
    }

    fn get_json(&self) -> String {
        todo!()
    }
}


struct ACSwitches {
    line_in: Box<TransmissionLines>,
    line_out:  Box<TransmissionLines>,
    open : bool
}

impl ACEquipment  for ACSwitches  {
    fn update(&mut self, elapsed_time: f32) {
        if (self.open){
            self.line_out.voltage.phase1 = self.line_in.voltage.phase1;
            self.line_out.voltage.phase2 = self.line_in.voltage.phase2;
            self.line_out.voltage.phase3 = self.line_in.voltage.phase3;
        } else {
            self.line_in.voltage.phase1 = 0.0;
            self.line_in.voltage.phase2 = 0.0;
            self.line_in.voltage.phase3 = 0.0;

            self.line_out.voltage.phase1 = 0.0;
            self.line_out.voltage.phase2 = 0.0;
            self.line_out.voltage.phase3 = 0.0;
        }
        self.line_out.update(elapsed_time);
        self.line_in.update(elapsed_time);
    }

    fn get_json(&self) -> String {
        todo!()
    }
}

struct TransmissionLines {
    current: Current3Phase,
    resistance: f32,
    voltage : Voltage3Phase
}

impl ACEquipment  for TransmissionLines {
    fn update(&mut self, _elapsed_time: f32) {
        self.current.phase1 = self.voltage.phase1/self.resistance;
        self.current.phase2 = self.voltage.phase2/self.resistance;
        self.current.phase3 = self.voltage.phase3/self.resistance;
    }

    fn get_json(&self) -> String {
        json!({"Type":"Transmission Line",
               "Voltages":{
                "Phase1":self.voltage.phase1,
                "Phase2":self.voltage.phase2,
                "Phase3":self.voltage.phase3
                },
               "Current":{
                "Phase1":self.current.phase1,
                "Phase2":self.current.phase2,
                "Phase3":self.current.phase3
                },
        }).to_string()
    }
}


struct Transformers<'a> {
    ratio : f32,
    primary: &'a Arc<Mutex<TransmissionLines>>,
    secondary: &'a Arc<Mutex<TransmissionLines>>
}

impl ACEquipment  for Transformers<'_>  {
    fn update(&mut self, elapsed_time: f32) {
        let mut secondary = self.secondary.lock().unwrap();
        let mut primary = self.primary.lock().unwrap();
        secondary.voltage.phase1 = primary.voltage.phase1*self.ratio;
        secondary.voltage.phase2 = primary.voltage.phase2*self.ratio;
        secondary.voltage.phase3 = primary.voltage.phase3*self.ratio;
        secondary.update(elapsed_time);
    }

    fn get_json(&self) -> String {
        json!({"Ratio":self.ratio}).to_string()
    }
}

struct Generators<'a> {
    voltage : Voltage3Phase,
    frequency : f32,
    line_out:  &'a Arc<Mutex<TransmissionLines>>,
    max_voltage : f32
}

impl ACEquipment  for Generators<'_>  {
    fn update(&mut self, elapsed_time: f32) {
        self.voltage.phase1 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time);
        self.voltage.phase2 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(std::f32::consts::FRAC_2_PI/3.0));
        self.voltage.phase3 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(2.0*std::f32::consts::FRAC_2_PI/3.0));

        let mut line_out = self.line_out.lock().unwrap();

        line_out.voltage.phase1 =  self.voltage.phase1;
        line_out.voltage.phase2 =  self.voltage.phase2;
        line_out.voltage.phase3 =  self.voltage.phase3;

        line_out.update(elapsed_time);
    }

    fn get_json(&self) -> String {
        json!({ "Voltages":{
                "Phase1":self.voltage.phase1,
                "Phase2":self.voltage.phase2,
                "Phase3":self.voltage.phase3
                },
            "Frequency" : self.frequency,
            "Max voltages" : self.max_voltage
        }).to_string()
    }
}

struct Voltage3Phase {
    phase1 : f32,
    phase2 : f32,
    phase3 : f32

}
struct Current3Phase {
    phase1 : f32,
    phase2 : f32,
    phase3 : f32
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

#[get("/start")]
fn start(grid: &State<Arc<Mutex<Location>>>) -> String {
    let mut elapsed_time = 0.1;
    loop {

        let mut grid = grid.lock().unwrap();

        for g in grid.equipment.iter_mut() {
            match g {
                GridEquipment::Consumer(c) => {c.update(elapsed_time)}
                GridEquipment::ACSwitches(_) => {}
                GridEquipment::Transformers(t) => {t.update(elapsed_time)}
                GridEquipment::Generators(g) => {g.update(elapsed_time)}
            }
        }
        elapsed_time += 0.1;
    }
}

#[launch]
fn rocket() -> _ {
    println!("Hi");
    let line1 = Arc::new(Mutex::new(TransmissionLines {
        current: Current3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
        resistance: 5.0,
        voltage: Voltage3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
    })) ;

    let line2 = Arc::new(Mutex::new(TransmissionLines {
        current: Current3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
        resistance: 5.0,
        voltage: Voltage3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
    }));


    let mut gen1 = Generators {
        voltage: Voltage3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
        frequency: 50.0,
        line_out: & line1,
        max_voltage: 230.0,
    };


    let mut trans1 = Transformers{
        ratio: 1.0,
        primary: &line1,
        secondary: &line2,
    };

    let mut house = Consumer { supplier:&line2, load: 0.0 };

    let mut power_test = Location { equipment: vec![GridEquipment::Generators(gen1),GridEquipment::Transformers(trans1),GridEquipment::Consumer(house)] };


    rocket::build()
        .mount("/", routes![index, produce, consume])
        .manage(Arc::new(Mutex::new(power_test)))
}
