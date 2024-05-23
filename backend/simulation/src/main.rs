mod grid_simulation;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    grid_simulation::test()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
