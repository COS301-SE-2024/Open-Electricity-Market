use crate::models::{NewProfileModel, NewUserModel, Profile, User};
use crate::{establish_connection, verify_user};
use diesel::prelude::*;
use pwhash::bcrypt;
use regex::Regex;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct NewUserRequest {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[post("/register", format = "application/json", data = "<new_user_request>")]
pub fn register(new_user_request: Json<NewUserRequest>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::profiles::dsl::*;
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut ret_session_id = "".to_string();

    let mut message = "Something went wrong".to_string();

    let mut email_valid = false;
    match Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        Ok(regex) => {
            email_valid = regex.is_match(&*new_user_request.email);
            if !email_valid {
                message = "Invalid email address".to_string()
            }
        }
        Err(_) => {}
    }

    let mut password_valid = false;
    if new_user_request.password.len() >= 8 {
        password_valid = true
    } else {
        message = "Password too short".to_string()
    }

    if email_valid && password_valid {
        let binding = bcrypt::hash(new_user_request.password.clone()).unwrap();

        let new_user_insert = NewUserModel {
            email: new_user_request.email.clone(),
            pass_hash: binding,
        };

        message = "Failed to create new user".to_string();
        match diesel::insert_into(users)
            .values(&new_user_insert)
            .returning(User::as_returning())
            .get_result::<User>(connection)
        {
            Ok(user) => {
                message = "Failed to update Session ID".to_string();
                let binding_2 =
                    bcrypt::hash(user.user_id.to_string() + &*user.created_at.to_string()).unwrap();
                match diesel::update(users)
                    .filter(user_id.eq(user.user_id))
                    .set(session_id.eq(binding_2))
                    .returning(User::as_returning())
                    .get_result(connection)
                {
                    Ok(user_up) => {
                        message = "Failed to add user profile".to_string();
                        let new_profile_insert = NewProfileModel {
                            profile_user_id: user.user_id,
                            first_name: new_user_request.first_name.clone(),
                            last_name: new_user_request.last_name.clone(),
                        };
                        match diesel::insert_into(profiles)
                            .values(&new_profile_insert)
                            .execute(connection)
                        {
                            Ok(_) => {
                                message = "New user added".to_string();
                                ret_session_id = user_up.session_id.clone().unwrap();
                                cookie_jar.add(
                                    Cookie::build(("session_id", user_up.session_id.unwrap()))
                                        .path("/"),
                                );
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({ "status": "ok", "message": message, "data": {"session_id": ret_session_id}})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Credentials {
    email: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>, jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut ret_session_id = "".to_string();

    let mut message = "Something went wrong".to_string();

    let mut email_valid = false;
    match Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        Ok(regex) => {
            email_valid = regex.is_match(&*credentials.email);
            if !email_valid {
                message = "Invalid email address".to_string()
            }
        }
        Err(_) => {}
    }

    if email_valid {
        match users
            .filter(email.eq(credentials.email.clone()))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                message = "Invalid password".to_string();
                let verify = bcrypt::verify(credentials.password.clone(), &*user.pass_hash);
                if verify {
                    match bcrypt::hash(user.user_id.to_string() + &*chrono::Utc::now().to_string())
                    {
                        Ok(hash) => {
                            match diesel::update(users)
                                .filter(email.eq(credentials.email.clone()))
                                .set(session_id.eq(hash.clone()))
                                .execute(connection)
                            {
                                Ok(_) => {
                                    message = "User logged in".to_string();
                                    ret_session_id = hash.clone();
                                    jar.add(Cookie::build(("session_id", hash)).path("/"));
                                    if !user.active {
                                        match diesel::update(users)
                                            .filter(user_id.eq(user.user_id))
                                            .set(active.eq(true))
                                            .execute(connection)
                                        {
                                            Ok(_) => {}
                                            Err(_) => {}
                                        }
                                    }
                                }
                                Err(_) => message = "Failed to update session id".to_string(),
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => message = "User does not exist".to_string(),
        }
    }

    json!({ "status": "ok", "message": message, "data": { "session_id": ret_session_id}})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RemoveFundsReq {
    funds: f64,
}

#[post(
    "/remove_funds",
    format = "application/json",
    data = "<remove_funds_req>"
)]
pub fn remove_funds(remove_funds_req: Json<RemoveFundsReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.user_id != Uuid::nil() {
        match users
            .filter(user_id.eq(claims.user_id))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                message = "Insufficient funds".to_string();
                if remove_funds_req.funds > 0f64 && user.credit >= remove_funds_req.funds {
                    match diesel::update(users)
                        .filter(user_id.eq(user.user_id))
                        .set(credit.eq(credit - remove_funds_req.funds))
                        .execute(connection)
                    {
                        Ok(_) => message = "Funds removed".to_string(),
                        Err(_) => message = "Failed to remove funds".to_string(),
                    }
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AddFundsReq {
    funds: f64,
}

#[post("/add_funds", format = "application/json", data = "<add_funds_req>")]
pub fn add_funds(add_funds_req: Json<AddFundsReq>, cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut message = claims.message.clone();

    if claims.user_id != Uuid::nil() {
        if add_funds_req.funds > 0f64 {
            match diesel::update(users)
                .filter(user_id.eq(claims.user_id))
                .set(credit.eq(credit + add_funds_req.funds))
                .execute(connection)
            {
                Ok(_) => message = "Funds added".to_string(),
                Err(_) => message = "Something went wrong.".to_string(),
            }
        }
    }

    json!({"status": "ok", "message": message})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserDetails {
    email: String,
    credit: f64,
    first_name: String,
    last_name: String,
}

#[post("/user_details")]
pub fn user_details(cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::profiles::dsl::*;
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let claims = verify_user(cookie_jar);

    let mut data = UserDetails {
        email: "".to_string(),
        credit: 0.0,
        first_name: "".to_string(),
        last_name: "".to_string(),
    };

    let mut message = claims.message.clone();

    if claims.message == "User found" {
        match users
            .filter(user_id.eq(claims.user_id))
            .select(User::as_select())
            .first(connection)
        {
            Ok(user) => {
                message = "No matching user profile".to_string();
                match profiles
                    .filter(profile_user_id.eq(user.user_id))
                    .select(Profile::as_select())
                    .first(connection)
                {
                    Ok(profile) => {
                        data = UserDetails {
                            email: user.email.clone(),
                            credit: user.credit,
                            first_name: profile.first_name.clone(),
                            last_name: profile.last_name.clone(),
                        };
                        message = "User details successfully retrieved".to_string();
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message, "data": data})
}

#[post("/remove_account")]
pub fn remove_account(cookie_jar: &CookieJar<'_>) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut message: String;

    let claims = verify_user(cookie_jar);
    message = claims.message.clone();

    if claims.message == "User found" {
        message = "Something went wrong".to_string();
        match diesel::update(users)
            .filter(user_id.eq(claims.user_id))
            .set(active.eq(false))
            .execute(connection)
        {
            Ok(_) => {
                message = "Account successfully deleted".to_string();
            }
            Err(_) => {}
        }
    }

    json!({"status": "ok", "message": message})
}
