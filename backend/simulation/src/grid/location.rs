use rocket::serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}
