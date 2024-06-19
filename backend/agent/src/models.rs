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