use std::{
    env,
    fmt::{format, write},
    result, thread, time, u32,
};

use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Clone, Copy)]
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
    circuit: u32,
    consumer: u32,
}

impl SmartMeterDetail {
    pub fn new() -> SmartMeterDetail {
        return SmartMeterDetail {
            circuit: 0,
            consumer: 0,
        };
    }
}

struct Node {
    node_id: String,
    location: Location,
    smart_meter: SmartMeter,
    generator: Generator,
}

impl Node {
    fn new(smart_meter: SmartMeter, generator: Generator) -> Node {
        return Node {
            node_id: String::new(),
            location: Location::new(),
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
        return SmartMeter::Acctive(ActiveSmartMeterCore {
            grid_detail: SmartMeterDetail::new(),
            consumption_curve,
        });
    }
}

struct ActiveSmartMeterCore {
    grid_detail: SmartMeterDetail,
    consumption_curve: Box<dyn Curve>,
}

enum Generator {
    Acctive(AcctiveGeneratorCore),
    InAcctive,
}

impl Generator {
    pub fn new_acctive(production_curve: Box<dyn Curve>) -> Generator {
        return Generator::Acctive(AcctiveGeneratorCore {
            grid_detail: GeneratorDetail::new(),
            production_curve,
        });
    }
}

struct AcctiveGeneratorCore {
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

#[derive(Serialize)]
struct LoginDetail {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterDetail {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(Deserialize)]
struct SessionWrapper {
    session_id: String,
}

#[derive(Deserialize)]
struct LoginResult {
    data: SessionWrapper,
    message: String,
    status: String,
}

#[derive(Deserialize)]
struct RegisterResult {
    data: SessionWrapper,
    message: String,
    status: String,
}

#[derive(Serialize)]
struct NodeDetail {
    name: String,
    location_x: f32,
    location_y: f32,
}

#[derive(Deserialize)]
struct NodeResult {
    message: String,
    status: String,
}

#[derive(Serialize)]
struct GetNodeDetail {
    limit: u32,
}

#[derive(Deserialize)]
struct NodeWrapper {
    name: String,
    node_id: String,
}

#[derive(Deserialize)]
struct GetNodeResult {
    data: Vec<NodeWrapper>,
    message: String,
    status: String,
}

#[derive(Deserialize)]
struct UserData {
    credit: f64,
    email: String,
    first_name: String,
    last_name: String,
}

#[derive(Deserialize)]
struct UserDetailResult {
    data: UserData,
    message: String,
    status: String,
}

#[derive(Serialize)]
struct NodeDetailsDetails {
    node_id: String,
}

#[derive(Deserialize)]
struct NodeDetailsData {
    location_x: f32,
    location_y: f32,
    name: String,
    node_id: String,
    units_to_consume: f64,
    units_to_produce: f64,
}

#[derive(Deserialize)]
struct NodeDetailsResult {
    data: NodeDetailsData,
    message: String,
    status: String,
}

struct Agent {
    email: String,
    password: String,
    session_id: String,
    nodes: Vec<Node>,
    units_bought: f64,
    units_sold: f64,
    funds: f64,
    extarnal_wealth_curve: Box<dyn Curve>,
}

impl Agent {
    fn new(
        email: String,
        password: String,
        nodes: Vec<Node>,
        funds: f64,
        extarnal_wealth_curve: Box<dyn Curve>,
    ) -> Agent {
        return Agent {
            email,
            password,
            session_id: String::from(""),
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

    fn login_or_register_agent(email: String, password: String) -> String {
        let login_detail = LoginDetail {
            email: email.clone(),
            password: password.clone(),
        };
        // let url = env::var("GURL").unwrap();
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/login"))
            .json(&login_detail)
            .send()
            .unwrap();
        let result: LoginResult = res.json().unwrap();
        if result.message == "User logged in" {
            return result.data.session_id;
        }
        let register_detail = RegisterDetail {
            email,
            first_name: String::from("Hal"),
            last_name: String::from("9000"),
            password,
        };
        let res = client
            .post(format!("http://{url}:8001/register"))
            .json(&register_detail)
            .send()
            .unwrap();
        let result: RegisterResult = res.json().unwrap();
        if result.message != "New user added" {
            panic!("Agent could not get session Id");
        }
        return result.data.session_id;
    }

    fn add_node(location: Location, name: String, session_id: String) {
        let node_detail = NodeDetail {
            name,
            location_x: location.latitude,
            location_y: location.longitude,
        };
        // let url = env::var("GURL").unwrap();
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/add_node"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&node_detail)
            .send()
            .unwrap();
        let result: NodeResult = res.json().unwrap();
        if result.message != "New Node Added" {
            println!("Could not add node")
        } else {
            println!("New node added");
        }
    }

    fn get_nodes(limit: u32, session_id: String) -> Vec<String> {
        let get_node_detail = GetNodeDetail { limit };
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/get_nodes"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&get_node_detail)
            .send()
            .unwrap();
        let result: GetNodeResult = res.json().unwrap();
        if result.message == "List of nodes successfully retrieved" {
            let mut out = vec![];

            for node in result.data {
                out.push(node.node_id);
            }
            return out;
        } else {
            return vec![];
        }
    }

    fn intialise(&mut self) {
        self.session_id = Agent::login_or_register_agent(self.email.clone(), self.password.clone());
        println!("{}", self.session_id.clone());
        let mut has_nodes = true;

        let mut node_ids = Agent::get_nodes(self.nodes.len() as u32, self.session_id.clone());
        if node_ids.len() == 0 {
            has_nodes = false;
        }

        for node in self.nodes.iter_mut() {
            //Create on grid
            match &mut node.generator {
                Generator::Acctive(core) => {
                    core.grid_detail = Agent::create_producer_grid(node.location);
                }
                Generator::InAcctive => {}
            }

            match &mut node.smart_meter {
                SmartMeter::Acctive(core) => {
                    core.grid_detail = Agent::create_consumer_grid(node.location);
                }
                SmartMeter::InActtive => {}
            }

            //Add nodes to market platform
            if !has_nodes {
                Agent::add_node(
                    node.location,
                    String::from("Simulated Node"),
                    self.session_id.clone(),
                )
            }
        }

        if !has_nodes {
            node_ids = Agent::get_nodes(self.nodes.len() as u32, self.session_id.clone());
        }

        let mut i = 0;
        for id in node_ids {
            if i >= self.nodes.len() {
                break;
            }
            self.nodes[i].node_id = id.clone();
            println!("{id}");
            i += 1;
        }
    }

    fn get_credit(session_id: String) -> f64 {
        // let url = env::var("GURL").unwrap();
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/user_details"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .send()
            .unwrap();
        let result: UserDetailResult = res.json().unwrap();
        if result.message == "User details successfully retrieved" {
            println!("Succesfully recieved credit {}", result.data.credit);
            return result.data.credit;
        } else {
            return 0.0;
        }
    }

    fn get_units_to_produce_and_consume(
        node_id: String,
        session_id: String,
    ) -> (Option<f64>, Option<f64>) {
        let node_details_details = NodeDetailsDetails { node_id };
        let url = "localhost";
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/node_details"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&node_details_details)
            .send()
            .unwrap();
        let result: NodeDetailsResult = res.json().unwrap();
        if result.message == "Node details retrieved succesfully" {
            let mut units_to_consume = None;
            if result.data.units_to_consume > 0.0 {
                units_to_consume = Some(result.data.units_to_consume);
            }
            let mut units_to_produce = None;
            if result.data.units_to_produce > 0.0 {
                units_to_produce = Some(result.data.units_to_produce);
            }
            return (units_to_consume, units_to_produce);
        } else {
            return (None, None);
        }
    }

    fn update(&mut self, accumlated_time: f64) -> Result<(), ()> {
        // get credit
        let credit = Agent::get_credit(self.session_id.clone());
        // uppdate credit based on income_curve
        self.funds += self.extarnal_wealth_curve.sample(accumlated_time);

        //foreach node
        for node in self.nodes.iter_mut() {
            // Get units_to_consume
            // Get units_to_produce
            let (units_to_consume, units_to_produce) = Agent::get_units_to_produce_and_consume(
                node.node_id.clone(),
                self.session_id.clone(),
            );

            // Update units_to_consume based on consumption curve
            let consumed = match &mut node.smart_meter {   
                SmartMeter::Acctive(core) => {
                match units_to_consume {
                    Some(to_consume) => {
                        let sample = core.consumption_curve.sample(accumlated_time);
                        if sample > to_consume {
                           to_consume
                        } else {
                            sample
                        }
                    },
                    None => {0.0},
                }
                },
                SmartMeter::InActtive => {0.0},
            };
            // Update units_to_produce based on produnction curve
            let produced = match &mut node.generator {
                Generator::Acctive(core) => {
                    match units_to_produce {
                        Some(to_produce) => {
                            let sample = core.production_curve.sample(accumlated_time);
                            if sample > to_produce {
                                to_produce
                            } else {
                                sample
                            }
                        },
                        None => {0.0},
                    }
                },
                Generator::InAcctive => {0.0},
            };
            
            // Update units_to_consume on market
            
            // Update units_to_produce on market

            // Set grid voltage for producer

            // Set grid impdence for consumer

            // Check if meet 24 hour requirment
            // buy electricity at market price
        }
        return Ok(());
    }

    fn run(&mut self) {
        self.intialise();
        loop {
            let accumlated_time = 0.0;
            let result = self.update(accumlated_time);

            match result {
                Ok(_) => thread::sleep(time::Duration::from_secs(15)),
                Err(_) => break,
            }
        }
    }
}

fn main() {
    let mut agent = Agent::new(
        String::from("a@example.com"),
        String::from("my_strong_password"),
        vec![Node::new(
            SmartMeter::new_acctive(Box::new(SineCurve::new())),
            Generator::new_acctive(Box::new(SineCurve::new())),
        )],
        0.0,
        Box::new(SineCurve::new()),
    );
    agent.run();
}
