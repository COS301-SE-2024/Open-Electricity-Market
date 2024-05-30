use diesel::prelude::*;
use diesel::sql_types::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::open_em::users)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub pass_hash: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::open_em::users)]
pub struct NewUserModel<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub pass_hash: &'a str
}
