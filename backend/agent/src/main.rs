use std::{env, thread, time};

use agent::Agent;
use curve::SineCurve;
use dotenvy::dotenv;
use generator::Generator;
use node::Node;
use smart_meter::SmartMeter;

pub mod agent;
pub mod curve;
pub mod generator;
pub mod location;
pub mod net_structs;
pub mod node;
pub mod smart_meter;

const AGENT_SPEED: u64 = 5 * 60;

fn main() {
    let mut handels = vec![];
    dotenv().ok();
    let password = env::var("PASSWORD").unwrap();

    for i in 1..15 {
        let password = password.clone();
        let handle = thread::spawn(move || {
            let mut agent = Agent::new(
                String::from(format!("{i}@example.com")),
                password,
                vec![Node::new(
                    SmartMeter::new_acctive(Box::new(SineCurve::new())),
                    Generator::new_acctive(Box::new(SineCurve::new())),
                )],
                0.0,
                Box::new(SineCurve::new()),
            );
            agent.run();
        });
        handels.push(handle);
        thread::sleep(time::Duration::from_secs(1 * AGENT_SPEED));
    }

    for handel in handels {
        handel.join().unwrap();
    }
}
