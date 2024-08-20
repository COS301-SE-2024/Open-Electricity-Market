#[macro_use]
extern crate rocket;

use std::{
    env,
    sync::{Arc, Mutex},
    thread, time,
};

use crate::tokio::time::sleep;
use agent::Agent;
use curve::{CummutiveCurve, SineCurve};
use dotenvy::dotenv;
use generator::{
    production_curve::{
        CoalGeneratorType, DieselGeneratorType, GeneratorCurve, GeneratorCurveType,
        HydraulicTurbineType, NuclearReactTypes, PetrolGeneratorType, SolarPanelType,
        WindTurbineType,
    },
    Generator,
};
use node::Node;
use rocket::serde::json::Json;
use rocket::{
    fairing::{Fairing, Info, Kind},
    State,
};
use rocket::{form::validate::Len, response::content, tokio};
use rocket::{
    http::{Header, Method, Status},
    Request, Response,
};
use serde::Deserialize;
use serde_json::json;
use smart_meter::consumption_curve::HomeApplianceType;
use smart_meter::{consumption_curve::HomeAppliance, SmartMeter};

pub mod agent;
pub mod curve;
pub mod generator;
pub mod location;
pub mod net_structs;
pub mod node;
pub mod period;
pub mod smart_meter;

const AGENT_SPEED: u64 = 5 * 60;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[post("/stats")]
fn stats() -> content::RawJson<String> {
    content::RawJson(json!({}).to_string())
}

#[post("/availible_generators")]
fn availible_generators() -> content::RawJson<String> {
    let all_generators = vec![
        GeneratorCurveType::SolarPanel(SolarPanelType::Home),
        GeneratorCurveType::SolarPanel(SolarPanelType::Industrial),
        GeneratorCurveType::WindTurbine(WindTurbineType::Small),
        GeneratorCurveType::WindTurbine(WindTurbineType::Medium),
        GeneratorCurveType::WindTurbine(WindTurbineType::Large),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::PWR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::BWR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::AGR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::FNR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::PHWR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::HTGR),
        GeneratorCurveType::NuclearReactor(NuclearReactTypes::LWGR),
        GeneratorCurveType::DieselGenerator(DieselGeneratorType::Home),
        GeneratorCurveType::DieselGenerator(DieselGeneratorType::Industrial),
        GeneratorCurveType::PetrolGenerator(PetrolGeneratorType::Home),
        GeneratorCurveType::PetrolGenerator(PetrolGeneratorType::Industrial),
        GeneratorCurveType::CoalGenerator(CoalGeneratorType::Small),
        GeneratorCurveType::CoalGenerator(CoalGeneratorType::Medium),
        GeneratorCurveType::CoalGenerator(CoalGeneratorType::Large),
        GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Small),
        GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Medium),
        GeneratorCurveType::HydraulicTurbine(HydraulicTurbineType::Large),
    ];
    content::RawJson(json!({"generators":all_generators}).to_string())
}

#[post("/availible_appliances")]
fn availible_appliances() -> content::RawJson<String> {
    let all_household_appliance = vec![
        HomeApplianceType::WashingMachine,
        HomeApplianceType::Router,
        HomeApplianceType::Vacuum,
        HomeApplianceType::Dishwasher,
        HomeApplianceType::Boiler,
        HomeApplianceType::HairPurifier,
        HomeApplianceType::SoundSystem,
        HomeApplianceType::Printer3d,
        HomeApplianceType::CoffeeMachine,
        HomeApplianceType::PhoneCharger,
        HomeApplianceType::Fridge,
        HomeApplianceType::Radiator,
        HomeApplianceType::Dehumidifier,
        HomeApplianceType::MicroWaveOven,
        HomeApplianceType::Laptop,
        HomeApplianceType::Tv,
        HomeApplianceType::Screen,
        HomeApplianceType::Fan,
        HomeApplianceType::AirConditioner,
        HomeApplianceType::Computer,
        HomeApplianceType::Printer,
        HomeApplianceType::Dryer,
        HomeApplianceType::Freezer,
    ];
    content::RawJson(json!({"Appliances":all_household_appliance}).to_string())
}

#[derive(Deserialize)]
struct AddApplianceDetail {
    email: String,
    node_id: String,
    appliances: Vec<HomeAppliance>,
}

#[post("/add_appliances", format = "application/json", data = "<data>")]
fn add_appliances(
    agents: &State<Arc<Mutex<Vec<Agent>>>>,
    data: Json<AddApplianceDetail>,
) -> content::RawJson<String> {
    let mut agents = agents.lock().unwrap();
    let agent_index = agents.iter().position(|agent| agent.email == data.email);
    if agent_index.is_none() {
        let message = "No agent exits asscioated with provide email";
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let agent_index = agent_index.unwrap();
    agents[agent_index].intialise();
    let node_index = agents[agent_index]
        .nodes
        .iter()
        .position(|node| node.node_id == data.node_id);
    if node_index.is_none() {
        let message = format!("Node {} does not exit", data.node_id);
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let node_index = node_index.unwrap();
    if let SmartMeter::InActtive = &mut agents[agent_index].nodes[node_index].smart_meter {
        agents[agent_index].nodes[node_index].smart_meter =
            SmartMeter::new_acctive(Box::new(CummutiveCurve::new()))
    }
    if let SmartMeter::Acctive(core) = &mut agents[agent_index].nodes[node_index].smart_meter {
        let data = data.into_inner();
        for appliance in data.appliances {
            core.consumption_curve.add_curve(Box::new(appliance));
        }
    }
    agents[agent_index].intialise();
    let message = "Succesfully added appliances".to_string();
    content::RawJson(json!({"status": "ok", "message": message, "data": {}}).to_string())
}

#[derive(Deserialize)]
struct AddGeneratrosDetail {
    email: String,
    node_id: String,
    generators: Vec<GeneratorCurve>,
}

#[post("/add_generators", format = "application/json", data = "<data>")]
fn add_generators(
    agents: &State<Arc<Mutex<Vec<Agent>>>>,
    data: Json<AddGeneratrosDetail>,
) -> content::RawJson<String> {
    let mut agents = agents.lock().unwrap();
    let agent_index = agents.iter().position(|agent| agent.email == data.email);
    if agent_index.is_none() {
        let message = "No agent exits asscioated with provide email";
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let agent_index = agent_index.unwrap();
    agents[agent_index].intialise();
    let node_index = agents[agent_index]
        .nodes
        .iter()
        .position(|node| node.node_id == data.node_id);
    if node_index.is_none() {
        let message = format!("Node {} does not exit", data.node_id);
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let node_index = node_index.unwrap();
    if let Generator::InAcctive = &mut agents[agent_index].nodes[node_index].generator {
        agents[agent_index].nodes[node_index].generator =
            Generator::new_acctive(Box::new(CummutiveCurve::new()))
    }
    if let Generator::Acctive(core) = &mut agents[agent_index].nodes[node_index].generator {
        let data = data.into_inner();
        for generator in data.generators {
            core.production_curve.add_curve(Box::new(generator));
        }
    }
    agents[agent_index].intialise();
    let message = "Succesfully added generators".to_string();
    content::RawJson(json!({"status": "ok", "message": message, "data": {}}).to_string())
}

#[derive(Deserialize)]
struct SetSessionDetail {
    email: String,
    session_id: String,
}

#[post("/set_session", format = "application/json", data = "<data>")]
fn set_session(
    agents: &State<Arc<Mutex<Vec<Agent>>>>,
    data: Json<SetSessionDetail>,
) -> content::RawJson<String> {
    let mut agents = agents.lock().unwrap();
    let agent_index = agents.iter().position(|agent| agent.email == data.email);
    if agent_index.is_none() {
        let message = "No agent exits asscioated with provide email";
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let agent_index = agent_index.unwrap();
    agents[agent_index].session_id = data.session_id.clone();

    let message = "Succesfully set session id".to_string();
    content::RawJson(json!({"status": "ok", "message": message, "data": {}}).to_string())
}

#[derive(Deserialize)]
struct AddAgentDetail {
    email: String,
    password: String,
    session_id: String,
}

#[post("/add_agent", format = "application/json", data = "<data>")]
fn add_agent(
    agents: &State<Arc<Mutex<Vec<Agent>>>>,
    data: Json<AddAgentDetail>,
) -> content::RawJson<String> {
    let lock = agents.inner().clone();
    let mut agents = agents.lock().unwrap();
    let agent_index = agents.iter().position(|agent| agent.email == data.email);
    if agent_index.is_some() {
        let message = "An agent acciotaed with that email already exits";
        return content::RawJson(
            json!({"status": "ok", "message": message, "data": {}}).to_string(),
        );
    }
    let data = data.into_inner();
    agents.push(Agent::new(
        data.email,
        data.password,
        vec![],
        0.0,
        true,
        Box::new(SineCurve::new()),
    ));

    let id = agents.len() - 1;
    agents[id].session_id = data.session_id;
    agents[id].intialise();
    tokio::spawn(async move {
        let mut accumilated_time = 0.0;
        loop {
            {
                let mut agent = lock.lock().unwrap();
                accumilated_time = agent[id].async_run(accumilated_time);
            }
            thread::sleep(time::Duration::from_secs(AGENT_SPEED));
        }
    });

    let message = "Succesfully added agent".to_string();
    content::RawJson(json!({"status": "ok", "message": message, "data": {}}).to_string())
}

#[launch]
fn rocket() -> _ {
    thread::spawn(move || {
        let mut handels = vec![];
        dotenv().ok();
        let password = env::var("PASSWORD").unwrap();

        for i in 1..15 {
            let password = password.clone();
            let handle = thread::spawn(move || {
                let mut agent = Agent::new(
                    format!("{i}@example.com"),
                    password,
                    vec![Node::new(
                        SmartMeter::new_acctive(Box::new(SineCurve::new())),
                        Generator::new_acctive(Box::new(SineCurve::new())),
                    )],
                    0.0,
                    false,
                    Box::new(SineCurve::new()),
                );
                agent.run();
            });
            handels.push(handle);
            thread::sleep(time::Duration::from_secs(AGENT_SPEED));
        }
    });

    rocket::build()
        .attach(CORS)
        .configure(rocket::Config::figment().merge(("port", 8002)))
        .mount(
            "/",
            routes![
                stats,
                availible_appliances,
                add_appliances,
                add_agent,
                availible_generators,
                add_generators,
                set_session
            ],
        )
        .manage(Arc::new(Mutex::new(Vec::<Agent>::new())))
}
