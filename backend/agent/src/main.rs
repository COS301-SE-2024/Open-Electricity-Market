use std::{
    env,
    fmt::{format, write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize,Clone,Copy)]
struct Location {
    latitude: f32,
    longitude: f32,
}

impl Location {
    pub fn new() -> Location {
        return Location {
            latitude: 0.0,
            longitude: 0.0,
        };
    }
}

#[derive(Deserialize)]
struct GeneratorDetail {
    circuit: u32,
    generator: u32,
}

impl GeneratorDetail {
    pub fn new() -> GeneratorDetail {
        return GeneratorDetail {
            circuit: 0,
            generator: 0,
        };
    }
}


#[derive(Deserialize)]
struct SmartMeterDetail {
    circuit : u32,
    consumer : u32
}

impl SmartMeterDetail {
    pub fn new() -> SmartMeterDetail {
        return SmartMeterDetail {
            circuit: 0,
            consumer: 0,
        }
    }
}

struct Node {
    node_id: String,
    smart_meter: SmartMeter,
    generator: Generator,
}

impl Node {
    fn new(smart_meter: SmartMeter, generator: Generator) -> Node {
        return Node {
            node_id: String::new(),
            smart_meter,
            generator,
        };
    }
}

enum SmartMeter {
    Acctive(ActiveSmartMeterCore),
    InActtive,
}

impl SmartMeter {
    pub fn new_acctive(consumption_curve: Box<dyn Curve>) -> SmartMeter {
        return SmartMeter::Acctive(ActiveSmartMeterCore{ location: Location::new(), grid_detail: SmartMeterDetail::new(), consumption_curve });
    }
}

struct ActiveSmartMeterCore {
    location : Location,
    grid_detail : SmartMeterDetail,
    consumption_curve: Box<dyn Curve>,
}

enum Generator {
    Acctive(AcctiveGeneratorCore),
    InAcctive,
}

impl Generator {
    pub fn new_acctive(production_curve: Box<dyn Curve>) -> Generator {
        return Generator::Acctive(AcctiveGeneratorCore {
            location: Location::new(),
            grid_detail: GeneratorDetail::new(),
            production_curve,
        });
    }
}

struct AcctiveGeneratorCore {
    location: Location,
    grid_detail: GeneratorDetail,
    production_curve: Box<dyn Curve>,
}

trait Curve {
    fn sample(&mut self, time: f64) -> f64;
}

struct SineCurve;

impl SineCurve {
    fn new() -> SineCurve {
        return SineCurve;
    }
}

impl Curve for SineCurve {
    fn sample(&mut self, time: f64) -> f64 {
        return f64::sin(time);
    }
}

struct Agent {
    nodes: Vec<Node>,
    units_bought: f64,
    units_sold: f64,
    funds: f64,
    extarnal_wealth_curve: Box<dyn Curve>,
}

impl Agent {
    fn new(nodes: Vec<Node>, funds: f64, extarnal_wealth_curve: Box<dyn Curve>) -> Agent {
        return Agent {
            nodes,
            units_bought: 0.0,
            units_sold: 0.0,
            funds,
            extarnal_wealth_curve,
        };
    }

    fn create_producer_grid(location: Location) -> GeneratorDetail {
        // let url = env::var("GURL").unwrap();
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8000/add_generator"))
            .json(&location)
            .send()
            .unwrap();
        let text = res.text().unwrap();
        println!("{}", &text);
        return serde_json::from_str(&text).unwrap();
    }

    fn create_consumer_grid(location: Location) -> SmartMeterDetail {
     // let url = env::var("GURL").unwrap();
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8000/add_consumer"))
            .json(&location)
            .send()
            .unwrap();
        let text = res.text().unwrap();
        println!("{}", &text);
        return serde_json::from_str(&text).unwrap();
    }

    fn intialise(&mut self) {
        for node in self.nodes.iter_mut() {
         //Create on grid
         match  &mut node.generator {
            Generator::Acctive(core) => {
               core.grid_detail = Agent::create_producer_grid(core.location);
            },
            Generator::InAcctive => {},
        }

        match &mut node.smart_meter {
            SmartMeter::Acctive(core) => {
                core.grid_detail = Agent::create_consumer_grid(core.location);
            },
            SmartMeter::InActtive => {},
        }
      
        //Add nodes to market platform
        }


   
    }

    fn update(&mut self) -> Result<(), ()> {
        return Ok(());
    }

    fn run(&mut self) {
        self.intialise();
        loop {
            let result = self.update();

            match result {
                Ok(_) => {}
                Err(_) => break,
            }
        }
    }
}

fn main() {
    let mut agent = Agent::new(
        vec![Node::new(
            SmartMeter::new_acctive(Box::new(SineCurve::new())),
            Generator::new_acctive(Box::new(SineCurve::new())),
        )],
        0.0,
        Box::new(SineCurve::new()),
    );
    agent.run();
}
