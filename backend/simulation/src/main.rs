#[macro_use]
extern crate rocket;

use std::cell::RefCell;
use std::cmp::max;
use rocket::State;
use std::sync::atomic::AtomicU64;
use std::thread::current;
use rocket::serde::json::json;


struct Location<'a> {
  equipment: Vec<GridEquipment<'a>>
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
    supplier:  &'a RefCell<TransmissionLines>,
    load : f32
}

impl ACEquipment for Consumer<'_> {
    fn update(&mut self, elapsed_time: f32) {
        let mut supplier_voltage =self.supplier.borrow_mut();

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
    primary: &'a RefCell<TransmissionLines>,
    secondary: &'a RefCell<TransmissionLines>
}

impl ACEquipment  for Transformers<'_>  {
    fn update(&mut self, elapsed_time: f32) {
        self.secondary.borrow_mut().voltage.phase1 = self.primary.borrow_mut().voltage.phase1*self.ratio;
        self.secondary.borrow_mut().voltage.phase2 = self.primary.borrow_mut().voltage.phase2*self.ratio;
        self.secondary.borrow_mut().voltage.phase3 = self.primary.borrow_mut().voltage.phase3*self.ratio;
        self.secondary.borrow_mut().update(elapsed_time);
    }

    fn get_json(&self) -> String {
        json!({"Ratio":self.ratio}).to_string()
    }
}

struct Generators<'a> {
    voltage : Voltage3Phase,
    frequency : f32,
    line_out:  &'a RefCell<TransmissionLines>,
    max_voltage : f32
}

impl ACEquipment  for Generators<'_>  {
    fn update(&mut self, elapsed_time: f32) {
        self.voltage.phase1 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time);
        self.voltage.phase2 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(std::f32::consts::FRAC_2_PI/3.0));
        self.voltage.phase3 = self.max_voltage*f32::sin(self.frequency*std::f32::consts::FRAC_2_PI*elapsed_time-(2.0*std::f32::consts::FRAC_2_PI/3.0));

        self.line_out.borrow_mut().voltage.phase1 =  self.voltage.phase1;
        self.line_out.borrow_mut().voltage.phase2 =  self.voltage.phase2;
        self.line_out.borrow_mut().voltage.phase3 =  self.voltage.phase3;

        self.line_out.borrow_mut().update(elapsed_time);
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

#[launch]
fn rocket() -> _ {
    println!("Hi");
    let line1 = RefCell::new(TransmissionLines {
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
    });

    let line2 = RefCell::new(TransmissionLines {
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
    });


    let mut gen1 = Generators {
        voltage: Voltage3Phase {
            phase1: 0.0,
            phase2: 0.0,
            phase3: 0.0,
        },
        frequency: 50.0,
        line_out: &line1,
        max_voltage: 230.0,
    };


    let mut trans1 = Transformers{
        ratio: 1.0,
        primary: &line1,
        secondary: &line2,
    };

    let mut house = Consumer { supplier:&line2, load: 0.0 };

    let mut elapsed_time = 0.1;

    loop {
        gen1.update(elapsed_time);
        trans1.update(elapsed_time);
        house.update(elapsed_time);
        elapsed_time += 0.1;
        println!("{}", line1.borrow().get_json());
    }

    rocket::build()
        .mount("/", routes![index, produce, consume])
        // .manage()
}
