use core::time;
use rand::Rng;
use reqwest::{blocking, header};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use serde_json::Value;
use std::time::Instant;
use std::{env, f64, thread};

use crate::location::Location;
use crate::{
    curve::Curve,
    generator::{Generator, GeneratorDetail},
    net_structs::*,
    node::Node,
    smart_meter::{SmartMeter, SmartMeterDetail},
    AGENT_SPEED,
};
#[derive(Serialize)]
pub struct Agent {
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub market_token: String,
    pub grid_token: String,
    pub nodes: Vec<Node>,
    pub funds: f64,
    pub linked_to_user: bool,
    pub extarnal_wealth_curve: Box<dyn Curve + Send + Sync>,
}

impl Agent {
    pub fn new(
        email: String,
        password: String,
        nodes: Vec<Node>,
        funds: f64,
        linked_to_user: bool,
        extarnal_wealth_curve: Box<dyn Curve + Send + Sync>,
    ) -> Agent {
        Agent {
            email,
            password,
            market_token: String::from(""),
            nodes,
            funds,
            linked_to_user,
            extarnal_wealth_curve,
            grid_token: String::from(""),
        }
    }

    fn create_producer_grid(
        location: Location,
        token: String,
        client: &reqwest::blocking::Client,
    ) -> GeneratorDetail {
        let url = env::var("GURL").unwrap();
        match client
            .post(format!("http://{url}:8000/add_generator"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&location)
            .send()
        {
            Ok(res) => {
                let text = res.text().unwrap();
                println!("{}", &text);
                serde_json::from_str(&text).unwrap()
            }
            Err(err) => {
                println!("Create Producer Grid {}", err);
                GeneratorDetail::new()
            }
        }
    }

    fn create_consumer_grid(
        location: Location,
        token: String,
        client: &reqwest::blocking::Client,
    ) -> SmartMeterDetail {
        let url = env::var("GURL").unwrap();

        match client
            .post(format!("http://{url}:8000/add_consumer"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&location)
            .send()
        {
            Ok(res) => {
                let text = res.text().unwrap();
                println!("{}", &text);
                serde_json::from_str(&text).unwrap()
            }
            Err(err) => {
                println!("Create Consumer Grid {}", err);
                return SmartMeterDetail::new();
            }
        }
    }

    fn login_or_register_agent(
        email: String,
        password: String,
        client: &reqwest::blocking::Client,
    ) -> String {
        let login_detail = LoginDetail {
            email: email.clone(),
            password: password.clone(),
        };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/login"))
            .json(&login_detail)
            .send()
        {
            Ok(res) => {
                let result: LoginResult = res.json().unwrap();
                if result.message == "User logged in" {
                    return result.data.token;
                }
                let register_detail = RegisterDetail {
                    email,
                    first_name: String::from("Hal"),
                    last_name: String::from("9000"),
                    password,
                };
                match client
                    .post(format!("http://{url}:8001/register"))
                    .json(&register_detail)
                    .send()
                {
                    Ok(res) => {
                        let result: RegisterResult = res.json().unwrap();
                        if result.message != "New user added" {
                            panic!("Agent could not get session Id");
                        }
                        result.data.token
                    }
                    Err(err) => {
                        println!("Login {}", err);
                        String::new()
                    }
                }
            }
            Err(err) => {
                println!("Register {}", err);
                String::new()
            }
        }
    }

    fn add_node(
        location: Location,
        name: String,
        token: String,
        client: &reqwest::blocking::Client,
    ) {
        let node_detail = NodeDetail {
            name,
            location_y: location.latitude,
            location_x: location.longitude,
        };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/add_node"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&node_detail)
            .send()
        {
            Ok(res) => {
                let result: NodeResult = res.json().unwrap();
                if result.message != "New Node Added" {
                    println!("Could not add node")
                } else {
                    println!("New node added");
                }
            }
            Err(err) => {
                println!("Add Node {}", err)
            }
        }
    }

    pub fn get_nodes(limit: u32, token: String, client: &reqwest::blocking::Client) -> Vec<String> {
        let get_node_detail = GetNodeDetail { limit };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/get_nodes"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&get_node_detail)
            .send()
        {
            Ok(res) => {
                let result: GetNodeResult = res.json().unwrap();
                if result.message == "List of nodes successfully retrieved" {
                    let mut out = vec![];

                    for node in result.data {
                        out.push(node.node_id);
                    }
                    out
                } else {
                    vec![]
                }
            }
            Err(err) => {
                println!("Get Nodes {}", err);
                vec![]
            }
        }
    }

    pub fn get_grid_token(email: String, client: &reqwest::blocking::Client) -> String {
        let password = env::var("GRID_PASS").unwrap();
        let detail = GetTokenDetail { email, password };
        let url = env::var("GURL").unwrap();

        match client
            .post(format!("http://{url}:8000/get_token"))
            .json(&detail)
            .send()
        {
            Ok(res) => {
                let result: GetTokenResult = res.json().unwrap();
                return result.token;
            }
            Err(err) => {
                println!("Get Token: {}", err);
                return String::new();
            }
        }
    }

    fn get_node_location(
        node_id: String,
        token: String,
        client: &reqwest::blocking::Client,
    ) -> Location {
        let node_details_details = NodeDetailsDetails { node_id };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/node_details"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&node_details_details)
            .send()
        {
            Ok(res) => {
                let result: NodeDetailsResult = res.json().unwrap();
                if result.message == "Node details retrieved succesfully" {
                    return Location {
                        latitude: result.data.location_x,
                        longitude: result.data.location_y,
                    };
                } else {
                    return Location::new();
                }
            }
            Err(err) => {
                println!("Get node location {}", err);
                return Location::new();
            }
        }
    }

    pub fn intialise(&mut self) {
        let client = blocking::Client::new();
        self.grid_token = Agent::get_grid_token(self.email.clone(), &client);
        self.market_token =
            Agent::login_or_register_agent(self.email.clone(), self.password.clone(), &client);

        if self.grid_token == "" || self.market_token == "" {
            return;
        }
        // println!("{}", self.market_token.clone());
        let mut has_nodes_on_market = true;

        let mut node_ids = Agent::get_nodes(1024, self.market_token.clone(), &client);
        if node_ids.is_empty() {
            has_nodes_on_market = false;
        }

        for node in self.nodes.iter_mut() {
            //Create on grid
            match &mut node.generator {
                Generator::Acctive(core) => {
                    if core.grid_detail.generator == 0 {
                        core.grid_detail = Agent::create_producer_grid(
                            node.location,
                            self.grid_token.clone(),
                            &client,
                        );
                    }
                }
                Generator::InAcctive => {}
            }

            match &mut node.smart_meter {
                SmartMeter::Acctive(core) => {
                    if core.grid_detail.consumer == 0 {
                        core.grid_detail = Agent::create_consumer_grid(
                            node.location,
                            self.grid_token.clone(),
                            &client,
                        );
                    }
                }
                SmartMeter::InActtive => {}
            }

            //Add nodes to market platform
            if !has_nodes_on_market {
                Agent::add_node(
                    node.location,
                    String::from("Simulated Node"),
                    self.market_token.clone(),
                    &client,
                )
            }
        }

        if !has_nodes_on_market {
            node_ids = Agent::get_nodes(1024, self.market_token.clone(), &client);
        }

        println!("Is empty {}", self.nodes.is_empty());
        if !self.nodes.is_empty() {
            if !self.linked_to_user {
                for (i, id) in node_ids.into_iter().enumerate() {
                    if i >= self.nodes.len() {
                        break;
                    }
                    self.nodes[i].node_id.clone_from(&id);
                    println!("{id}");
                }
            } else {
                for id in node_ids.iter() {
                    let pos = self.nodes.iter().position(|n| n.node_id == *id);
                    match pos {
                        Some(_) => {}
                        None => {
                            self.nodes.push(Node {
                                node_id: id.clone(),
                                location: Agent::get_node_location(
                                    id.clone(),
                                    self.market_token.clone(),
                                    &client,
                                ),
                                smart_meter: SmartMeter::InActtive,
                                generator: Generator::InAcctive,
                            });
                            println!("{id}");
                        }
                    }
                }
            }
        } else {
            for id in node_ids.iter() {
                if !self.linked_to_user {
                    self.nodes.push(Node {
                        node_id: id.clone(),
                        location: Location::new(),
                        smart_meter: SmartMeter::InActtive,
                        generator: Generator::InAcctive,
                    });
                    println!("{id}");
                } else {
                    self.nodes.push(Node {
                        node_id: id.clone(),
                        location: Agent::get_node_location(
                            id.clone(),
                            self.market_token.clone(),
                            &client,
                        ),
                        smart_meter: SmartMeter::InActtive,
                        generator: Generator::InAcctive,
                    });
                    println!("{id}");
                }
            }
        }
    }

    fn get_credit(token: String, client: &reqwest::blocking::Client) -> f64 {
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/user_details"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .send()
        {
            Ok(res) => {
                let result: UserDetailResult = res.json().unwrap();
                if result.message == "User details successfully retrieved" {
                    println!("Succesfully recieved credit {}", result.data.credit);
                    result.data.credit
                } else {
                    0.0
                }
            }
            Err(err) => {
                println!("Get credit {}", err);
                0.0
            }
        }
    }

    fn get_units_to_produce_and_consume(
        node_id: String,
        token: String,
        client: &reqwest::blocking::Client,
    ) -> (Option<f64>, Option<f64>) {
        let node_details_details = NodeDetailsDetails { node_id };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/node_details"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&node_details_details)
            .send()
        {
            Ok(res) => {
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
                    (units_to_consume, units_to_produce)
                } else {
                    (None, None)
                }
            }
            Err(err) => {
                println!("Get units to produce and consume {}", err);
                (None, None)
            }
        }
    }

    fn update_units_consumed(
        units: f64,
        token: String,
        node_id: String,
        client: &reqwest::blocking::Client,
    ) {
        let update_units_consumed_details = UpdateUnitsConsumedDetails { units, node_id };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/update_consumed_units"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&update_units_consumed_details)
            .send()
        {
            Ok(res) => {
                let result: UpdateUnitsConsumeResult = res.json().unwrap();
                println!("{}", result.message);
            }
            Err(err) => {
                println!("Updated units counsumed {}", err)
            }
        }
    }

    fn update_units_produced(
        units: f64,
        token: String,
        node_id: String,
        client: &reqwest::blocking::Client,
    ) {
        let update_units_consumed_details = UpdateUnitsProducedDetails { units, node_id };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/update_produced_units"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&update_units_consumed_details)
            .send()
        {
            Ok(res) => {
                let result: UpdateUnitsProducedResult = res.json().unwrap();
                println!("{}", result.message);
            }
            Err(err) => {
                println!("Updated units produced {}", err)
            }
        }
    }

    fn update_grid_voltage(
        units: f64,
        detail: GeneratorDetail,
        token: String,
        client: &reqwest::blocking::Client,
    ) {
        let voltage_update_detail = VoltageUpdateDetail {
            circuit: detail.circuit,
            generator: detail.generator,
            power: units as f32,
        };
        let url = env::var("GURL").unwrap();

        match client
            .post(format!("http://{url}:8000/set_generator"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&voltage_update_detail)
            .send()
        {
            Ok(res) => {
                let result: VoltageUpdateResult = res.json().unwrap();
                println!("{} set to {}", result.message, units);
            }
            Err(err) => {
                println!("Updated grid voltage {}", err)
            }
        }
    }

    fn update_grid_impedance(
        units: f64,
        detail: SmartMeterDetail,
        token: String,
        client: &reqwest::blocking::Client,
    ) {
        let impedance_update_detail = ImpedanceUpdateDetail {
            circuit: detail.circuit,
            consumer: detail.consumer,
            power: units as f32,
        };
        let url = env::var("GURL").unwrap();

        match client
            .post(format!("http://{url}:8000/set_consumer"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&impedance_update_detail)
            .send()
        {
            Ok(res) => {
                let result: ImpedanceUpdateResult = res.json().unwrap();
                println!("{}", result.message);
            }
            Err(err) => {
                println!("Updated grid impedance {}", err)
            }
        }
    }

    fn get_current_price(client: &reqwest::blocking::Client) -> f64 {
        // let url = env::var("GURL").unwrap();
        let url = env::var("MURL").unwrap();

        match client.post(format!("http://{url}:8001/price_view")).send() {
            Ok(res) => {
                let result: GetPriceResult = res.json().unwrap();
                if result.message == "Successfully retrieved price" {
                    println!("Succesfully recieved price {}", result.data.price);
                    result.data.price
                } else {
                    0.003
                }
            }
            Err(err) => {
                println!("Get current price {}", err);
                0.0
            }
        }
    }

    fn place_buy_order(
        token: String,
        node_id: String,
        mut units: f64,
        funds: f64,
        client: &reqwest::blocking::Client,
    ) -> f64 {
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen_range(-0.0010..0.0010);

        let mut market_price = Agent::get_current_price(client) + offset;

        if market_price <= 0.0 {
            market_price = 0.003;
        }

        let ratio = funds / (market_price * units);
        if ratio < 1.0 {
            units *= ratio;
        }

        let detail = PlaceBuyOrderDetail {
            node_id,
            price: market_price,
            units,
        };

        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/buy_order"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&detail)
            .send()
        {
            Ok(res) => {
                let result: BuyOrderResult = res.json().unwrap();

                if result.message == "Buy order created successfully. Pending match"
                    || result.message == "Buy order created successfully. Order match"
                {
                    println!("Buy order place for {}", units);
                    market_price * units
                } else {
                    0.0
                }
            }
            Err(err) => {
                println!("Place buy order {}", err);
                return 0.0;
            }
        }
    }

    fn place_sell_order(
        token: String,
        node_id: String,
        units: f64,
        client: &reqwest::blocking::Client,
    ) {
        let mut rng = rand::thread_rng();
        let offset: f64 = rng.gen_range(-0.0010..0.0010);

        let market_price = Agent::get_current_price(client) + offset;

        let detail = PlaceSellOrderDetail {
            node_id,
            price: market_price,
            units,
        };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/sell_order"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&detail)
            .send()
        {
            Ok(res) => {
                let result: SellOrderResult = res.json().unwrap();
                if result.message == "Sell order created successfully. Pending match"
                    || result.message == "Sell order created successfully. Order match"
                {
                    println!("Placed Sell Order for {}", units)
                }
            }
            Err(err) => {
                println!("Place sell order {}", err);
            }
        }
    }

    fn update_credit(token: String, amount: f64, client: &reqwest::blocking::Client) {
        println!("trying to add {amount} credit");
        let detail = AddFundDetails { funds: amount };
        let url = env::var("MURL").unwrap();

        match client
            .post(format!("http://{url}:8001/add_funds"))
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .json(&detail)
            .send()
        {
            Ok(res) => {
                let result: AddFundResult = res.json().unwrap();
                println!("{}", result.message.clone());
                if result.message == "Funds added" {
                    println!("Added {amount} credit")
                }
            }
            Err(err) => {
                println!("Update credit {}", err);
            }
        }
    }

    fn update(&mut self, accumlated_time: f64) -> Result<(), ()> {
        let client = blocking::Client::new();

        self.grid_token = Agent::get_grid_token(self.email.clone(), &client);
        self.market_token =
            Agent::login_or_register_agent(self.email.clone(), self.password.clone(), &client);

        if self.grid_token == "" || self.market_token == "" {
            return Ok(());
        }

        // update credit based on income_curve
        if !self.linked_to_user {
            Agent::update_credit(
                self.market_token.clone(),
                self.extarnal_wealth_curve.sample(accumlated_time),
                &client,
            );
        }

        // get credit
        let credit = Agent::get_credit(self.market_token.clone(), &client);

        //foreach node
        for node in self.nodes.iter_mut() {
            // Get units_to_consume
            // Get units_to_produce
            let (units_to_consume, units_to_produce) = Agent::get_units_to_produce_and_consume(
                node.node_id.clone(),
                self.market_token.clone(),
                &client,
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
                    self.market_token.clone(),
                    node.node_id.clone(),
                    &client,
                );
            }

            // Update units_to_produce on market
            if produced > 0.0 {
                Agent::update_units_produced(
                    produced,
                    self.market_token.clone(),
                    node.node_id.clone(),
                    &client,
                );
            }

            // Set grid voltage for producer
            match &node.generator {
                Generator::Acctive(core) => {
                    if produced > 0.0 {
                        Agent::update_grid_voltage(
                            produced,
                            core.grid_detail,
                            self.grid_token.clone(),
                            &client,
                        )
                    }
                }
                Generator::InAcctive => {}
            }

            // Set grid impdence for consumer
            match &node.smart_meter {
                SmartMeter::Acctive(core) => {
                    if consumed > 0.0 {
                        Agent::update_grid_impedance(
                            consumed,
                            core.grid_detail,
                            self.grid_token.clone(),
                            &client,
                        )
                    }
                }
                SmartMeter::InActtive => {}
            }

            if !self.linked_to_user {
                // Check if meet 24 hour requirment
                match &mut node.smart_meter {
                    SmartMeter::Acctive(core) => {
                        let to_consume = units_to_consume.unwrap_or(0.0);
                        let gap =
                            core.consumption_curve.total_in_24_hour() - (to_consume - consumed);
                        println!("{}", gap);
                        if gap > 0.0 && credit > 0.0 {
                            // buy electricity at market price
                            let spent = Agent::place_buy_order(
                                self.market_token.clone(),
                                node.node_id.clone(),
                                gap,
                                credit,
                                &client,
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
                        let to_produce = units_to_produce.unwrap_or(0.0);
                        let produced = core.production_curve.sample(accumlated_time) - produced;
                        if produced > to_produce {
                            Agent::place_sell_order(
                                self.market_token.clone(),
                                node.node_id.clone(),
                                produced - to_produce,
                                &client,
                            );
                        }
                    }
                    Generator::InAcctive => {}
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self, autos: Arc<Mutex<Vec<Value>>>, val_id: usize) {
        self.intialise();

        let mut accumlated_time = 0.0;
        let mut elapsed;
        loop {
            let now = Instant::now();
            let result = self.update(accumlated_time);
            elapsed = now.elapsed().as_secs_f64();
            accumlated_time += elapsed;
            let santas_address: serde_json::Value =
                serde_json::from_str(&serde_json::to_string(self).unwrap()).expect("REASON");
            {
                let mut autos = autos.lock().unwrap();
                autos[val_id] = santas_address;
            }

            match result {
                Ok(_) => thread::sleep(time::Duration::from_secs(15 * AGENT_SPEED)),
                Err(_) => break,
            }
        }
    }

    pub fn async_run(&mut self, mut accumilated_time: f64) -> f64 {
        let now = Instant::now();
        let _ = self.update(accumilated_time);
        let elapsed = now.elapsed().as_secs_f64();
        accumilated_time += elapsed;
        accumilated_time
    }
}
