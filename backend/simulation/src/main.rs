mod grid_simulation;

#[macro_use] extern crate rocket;

use std::fmt::format;
use std::sync::atomic::{AtomicU64, Ordering};
use rocket::State;

struct HitCount {
    count: AtomicU64
}

#[get("/")]
fn index(state: &State<grid_simulation::Grid>) -> String {
    // let current_count = hit_count.count.fetch_add(1,Ordering::Relaxed);
    let curren_voltage = state.grid.get_avg_distribution_line_voltage();
    format!("Number of visits: {}", curren_voltage)
}


#[get("/produce")]
fn produce() -> String{
    format!("{}","produce")
}

#[get("/consume")]
fn consume() -> String{
    format!("{}","consume")
}

#[launch]
fn rocket() -> _ {
    let d1 = grid_simulation::DistributionLine{resistance: 150,amps :AtomicU64::new(0),to : grid_simulation::GridPiece::Nil};
    rocket::build().mount("/", routes![index,produce,consume]).manage(grid_simulation::Grid{grid : Box::new(d1)})

}