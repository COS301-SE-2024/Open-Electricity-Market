use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub pass_hash: String,
    pub credit: f64,
    pub units_bought: f64,
    pub units_sold: f64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::users)]
pub struct NewUserModel<'a> {
    pub email: &'a str,
    pub pass_hash: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::profiles)]
pub struct NewProfileModel<'a>{
    pub user_id: &'a Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
}