use std::{env, fmt::format};

use serde::{Deserialize, Serialize};

struct Node {
    node_id: String,
    smart_meter: Option<SmartMeter>,
    generator: Option<Generator>,
}

impl Node {
    fn new(smart_meter: Option<SmartMeter>, generator: Option<Generator>) -> Node {
        return Node {
            node_id: String::new(),
            smart_meter,
            generator,
        };
    }
}

struct SmartMeter {
    consumption_curve: Box<dyn Curve>,
}

struct Generator {
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

#[derive(Serialize)]
struct Location {
    latitude: f32,
    longitude: f32,
}


#[derive(Deserialize)]
struct GeneratorDetail {
   circuit : u32,
   generator : u32
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
        let url = env::var("GURL").unwrap();
        let message = serde_json::to_string(&location).unwrap();

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8000/add_generator"))
            .body(message)
            .send()
            .unwrap();
        
        return serde_json::from_str(&res.text().unwrap()).unwrap();
    }

    fn intialise(&mut self) {
        //Create on grid
        Agent::create_producer_grid(Location {latitude: 15.0, longitude: 15.0 });

        //Add nodes to market platform
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
    let mut agent = Agent::new(vec![Node::new(None, None)], 0.0, Box::new(SineCurve::new()));
    agent.run();
}
