use core::time;
use rand::Rng;
use reqwest::header;
use std::{env, thread};

use crate::{
    curve::Curve,
    generator::{Generator, GeneratorDetail},
    location::Location,
    net_structs::*,
    node::Node,
    smart_meter::{SmartMeter, SmartMeterDetail},
    AGENT_SPEED,
};

pub struct Agent {
    pub email: String,
    pub password: String,
    pub session_id: String,
    pub nodes: Vec<Node>,
    pub funds: f64,
    pub extarnal_wealth_curve: Box<dyn Curve>,
}

impl Agent {
    pub fn new(
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
            funds,
            extarnal_wealth_curve,
        };
    }

    fn create_producer_grid(location: Location) -> GeneratorDetail {
        let url = env::var("GURL").unwrap();
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
        let url = env::var("GURL").unwrap();
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
        let url = env::var("MURL").unwrap();
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
        let url = env::var("MURL").unwrap();
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
        let url = env::var("MURL").unwrap();
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
        let url = env::var("MURL").unwrap();
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
        let url = env::var("MURL").unwrap();
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
            if result.data.units_to_consume > 0.1 {
                units_to_consume = Some(result.data.units_to_consume);
            }
            let mut units_to_produce = None;
            if result.data.units_to_produce > 0.1 {
                units_to_produce = Some(result.data.units_to_produce);
            }
            return (units_to_consume, units_to_produce);
        } else {
            return (None, None);
        }
    }

    fn update_units_consumed(units: f64, session_id: String, node_id: String) {
        let update_units_consumed_details = UpdateUnitsConsumedDetails { units, node_id };
        let url = env::var("MURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/update_consumed_units"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&update_units_consumed_details)
            .send()
            .unwrap();
        let result: UpdateUnitsConsumeResult = res.json().unwrap();
        println!("{}", result.message);
    }

    fn update_units_produced(units: f64, session_id: String, node_id: String) {
        let update_units_consumed_details = UpdateUnitsProducedDetails { units, node_id };
        let url = env::var("MURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/update_produced_units"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&update_units_consumed_details)
            .send()
            .unwrap();
        let result: UpdateUnitsProducedResult = res.json().unwrap();
        println!("{}", result.message);
    }

    fn update_grid_voltage(units: f64, detail: GeneratorDetail) {
        let voltage_update_detail = VoltageUpdateDetail {
            circuit: detail.circuit,
            generator: detail.generator,
            power: units as f32,
        };
        let url = env::var("GURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8000/set_generator"))
            .json(&voltage_update_detail)
            .send()
            .unwrap();
        let result: VoltageUpdateResult = res.json().unwrap();
        println!("{}", result.message);
    }

    fn update_grid_impedance(units: f64, detail: SmartMeterDetail) {
        let impedance_update_detail = ImpedanceUpdateDetail {
            circuit: detail.circuit,
            consumer: detail.consumer,
            power: units as f32,
        };
        let url = env::var("GURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8000/set_consumer"))
            .json(&impedance_update_detail)
            .send()
            .unwrap();
        let result: ImpedanceUpdateResult = res.json().unwrap();
        println!("{}", result.message);
    }

    fn get_current_price() -> f64 {
        // let url = env::var("GURL").unwrap();
        let url = env::var("MURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/price_view"))
            .send()
            .unwrap();
        let result: GetPriceResult = res.json().unwrap();
        if result.message == "Successfully retrieved price" {
            println!("Succesfully recieved price {}", result.data.price);
            return result.data.price;
        } else {
            return 100.0;
        }
    }

    fn place_buy_order(session_id: String, node_id: String, mut units: f64, funds: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen_range(-15.0..15.0);

        let market_price = Agent::get_current_price() + offset;

        let max_price = market_price + 10.0;

        let ratio = funds / (max_price * units);
        if ratio < 1.0 {
            units = ratio * units;
        }

        let detail = PlaceBuyOrderDetail {
            node_id,
            min_price: market_price - 10.0,
            max_price: market_price + 10.0,
            units,
        };

        let url = env::var("MURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/buy_order"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&detail)
            .send()
            .unwrap();
        let result: BuyOrderResult = res.json().unwrap();

        if result.message == "Buy order created successfully. Pending match"
            || result.message == "Buy order created successfully. Order match"
        {
            println!("Buy order place for {}", units);
            return market_price * units;
        } else {
            return 0.0;
        }
    }

    fn place_sell_order(session_id: String, node_id: String, units: f64) {
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen_range(-15.0..15.0);

        let market_price = Agent::get_current_price() + offset;

        let detail = PlaceSellOrderDetail {
            node_id,
            min_price: market_price - 10.0,
            max_price: market_price + 10.0,
            units,
        };
        let url = env::var("MURL").unwrap();
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(format!("http://{url}:8001/sell_order"))
            .header(header::COOKIE, format!("session_id={session_id}"))
            .json(&detail)
            .send()
            .unwrap();
        let result: SellOrderResult = res.json().unwrap();
        if result.message == "Sell order created successfully. Pending match"
            || result.message == "Sell order created successfully. Order match"
        {
            println!("Placed Sell Order for {}", units)
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
                SmartMeter::Acctive(core) => match units_to_consume {
                    Some(to_consume) => {
                        let sample = core.consumption_curve.sample(accumlated_time);
                        if sample > to_consume {
                            to_consume
                        } else {
                            sample
                        }
                    }
                    None => 0.0,
                },
                SmartMeter::InActtive => 0.0,
            };
            // Update units_to_produce based on produnction curve
            let produced = match &mut node.generator {
                Generator::Acctive(core) => match units_to_produce {
                    Some(to_produce) => {
                        let sample = core.production_curve.sample(accumlated_time);
                        if sample > to_produce {
                            to_produce
                        } else {
                            sample
                        }
                    }
                    None => 0.0,
                },
                Generator::InAcctive => 0.0,
            };

            // Update units_to_consume on market
            if consumed > 0.0 {
                Agent::update_units_consumed(
                    consumed,
                    self.session_id.clone(),
                    node.node_id.clone(),
                );
            }

            // Update units_to_produce on market
            if produced > 0.0 {
                Agent::update_units_produced(
                    produced,
                    self.session_id.clone(),
                    node.node_id.clone(),
                );
            }

            // Set grid voltage for producer
            match &node.generator {
                Generator::Acctive(core) => {
                    if produced > 0.0 {
                        Agent::update_grid_voltage(produced, core.grid_detail)
                    }
                }
                Generator::InAcctive => {}
            }

            // Set grid impdence for consumer
            match &node.smart_meter {
                SmartMeter::Acctive(core) => {
                    if consumed > 0.0 {
                        Agent::update_grid_impedance(consumed, core.grid_detail)
                    }
                }
                SmartMeter::InActtive => {}
            }

            // Check if meet 24 hour requirment
            match &mut node.smart_meter {
                SmartMeter::Acctive(core) => {
                    let to_consume = match units_to_consume {
                        Some(to_consume) => to_consume,
                        None => 0.0,
                    };
                    let gap = core.consumption_curve.total_in_24_hour() - (to_consume - consumed);
                    println!("{}", gap);
                    if gap > 0.0 && credit > 0.0 {
                        // buy electricity at market price
                        let spent = Agent::place_buy_order(
                            self.session_id.clone(),
                            node.node_id.clone(),
                            gap,
                            credit,
                        );

                        if spent > credit {
                            let intermediate = spent - credit;
                            self.funds -= intermediate;
                        }
                    }
                }
                SmartMeter::InActtive => {}
            }

            match &mut node.generator {
                Generator::Acctive(core) => {
                    let to_produce = match units_to_produce {
                        Some(to_produce) => to_produce,
                        None => 0.0,
                    };
                    let produced = core.production_curve.sample(accumlated_time) - produced;
                    if produced > to_produce {
                        Agent::place_sell_order(
                            self.session_id.clone(),
                            node.node_id.clone(),
                            produced - to_produce,
                        );
                    }
                }
                Generator::InAcctive => {}
            }
        }
        return Ok(());
    }

    pub fn run(&mut self) {
        self.intialise();
        loop {
            let accumlated_time = 1.0;
            let result = self.update(accumlated_time);

            match result {
                Ok(_) => thread::sleep(time::Duration::from_secs(15 * AGENT_SPEED)),
                Err(_) => break,
            }
        }
    }
}
