

mod grid_simulation;

#[macro_use] extern crate rocket;

use std::sync::atomic::{AtomicU64};
use rocket::State;


#[get("/")]
fn index(state: &State<grid_simulation::Grid>) -> String {
    // let current_count = hit_count.count.fetch_add(1,Ordering::Relaxed);
    let curren_voltage = state.grid.get_avg_distribution_line_voltage();
    format!("{curren_voltage}")
}


#[get("/produce/<amount>")]
fn produce(state: &State<grid_simulation::Grid>,amount: u64) -> String{
    state.grid.produce(amount);
    format!("{}","produce")
}

#[get("/consume/<amount>")]
fn consume(state: &State<grid_simulation::Grid>,amount: u64) -> String{
    let c = state.grid.consume(amount);
    format!("Consume {c}")
}

#[launch]
fn rocket() -> _ {
    let d1 = grid_simulation::DistributionLine{resistance: 0.2,amps :AtomicU64::new(0),to : grid_simulation::GridPiece::Nil};
    rocket::build().mount("/", routes![index,produce,consume]).manage(grid_simulation::Grid{grid : Box::new(d1)})

}


