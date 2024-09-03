use crate::models::{NewProfileModel, NewUserModel, Profile, User};
use crate::{establish_connection, TOKEN_EXPIRATION};
use chrono::Utc;
use diesel::prelude::*;
use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use pwhash::bcrypt;
use regex::Regex;
use rocket::{
    http::{CookieJar, Status},
    request::{FromRequest, Outcome},
    response::status::Custom,
    serde::{
        json::{serde_json::json, Json, Value},
        Deserialize, Serialize,
    },
};
use std::env;
use uuid::Uuid;

const BEARER: &str = "Bearer ";
const AUTHORIZATION: &str = "Authorization";

// Used when decoding a token to `Claims`
#[derive(Debug, PartialEq)]
pub enum AuthenticationError {
    Missing,
    Decoding(String),
    Expired,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = AuthenticationError;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one(AUTHORIZATION) {
            None => Outcome::Error((Status::Forbidden, AuthenticationError::Missing)),
            Some(value) => match Claims::from_authorization(value) {
                Ok(claims) => Outcome::Success(claims),
                Err(e) => Outcome::Error((Status::Forbidden, e)),
            },
        }
    }
}

impl Claims {
    pub fn from_name(user_id: String) -> Self {
        Self { user_id, exp: 0 }
    }

    fn from_authorization(value: &str) -> Result<Self, AuthenticationError> {
        dotenv().ok();

        let token = value.strip_prefix(BEARER).map(str::trim);

        if token.is_none() {
            return Err(AuthenticationError::Missing);
        }

        let token = token.unwrap();

        let token = decode::<Claims>(
            token,
            &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            ErrorKind::ExpiredSignature => AuthenticationError::Expired,
            _ => AuthenticationError::Decoding(e.to_string()),
        })?;

        Ok(token.claims)
    }

    pub(crate) fn into_token(mut self) -> Result<String, Custom<String>> {
        dotenv().ok();

        let expiration = Utc::now()
            .checked_add_signed(TOKEN_EXPIRATION)
            .expect("failed to create an expiration time")
            .timestamp();

        self.exp = expiration as usize;

        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
        )
        .map_err(|e| Custom(Status::BadRequest, e.to_string()))?;

        Ok(token)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUserRequest {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[post("/register", format = "application/json", data = "<new_user_request>")]
pub fn register(new_user_request: Json<NewUserRequest>) -> Value {
    use crate::schema::open_em::profiles::dsl::*;
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

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
                        let claim = Claims::from_name(user.user_id.to_string());
                        match claim.into_token() {
                            Ok(token) => {
                                return json!({"status": "ok",
                                    "message": "New user added".to_string(),
                                    "data": {"token": token}
                                })
                            }
                            Err(_) => {
                                return json!({"status": "error",
                                    "message": "Something went wrong".to_string(),
                                    "data": {"token": "".to_string()}
                                })
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    json!({ "status": "error", "message": message, "data": {"token": "".to_string()}})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
    email: String,
    password: String,
}

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn login(credentials: Json<Credentials>) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    match Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-.][a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        Ok(_) => {}
        Err(_) => {
            return json!({ "status": "error",
                "message": "Email address format invalid",
                "data": { "token": "".to_string()}
            })
        }
    }

    match users
        .filter(email.eq(credentials.email.clone()))
        .select(User::as_select())
        .first(connection)
    {
        Ok(user) => {
            if bcrypt::verify(credentials.password.clone(), &*user.pass_hash) {
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
                let claim = Claims::from_name(user.user_id.to_string());
                match claim.into_token() {
                    Ok(token) => {
                        return json!({"status": "ok",
                            "message": "User logged in".to_string(),
                            "data": {"token": token}
                        })
                    }
                    Err(_) => {
                        return json!({"status": "error",
                            "message": "Something went wrong".to_string(),
                            "data": {"token": "".to_string()}
                        })
                    }
                }
            }
            json!({ "status": "error",
                "message": "Username or password invalid".to_string(),
                "data": { "token": "".to_string()}
            })
        }
        Err(_) => json!({ "status": "error",
            "message": "Username or password invalid".to_string(),
            "data": { "token": "".to_string()}
        }),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RemoveFundsReq {
    funds: f64,
}

#[post(
    "/remove_funds",
    format = "application/json",
    data = "<remove_funds_req>"
)]
pub fn remove_funds(remove_funds_req: Json<RemoveFundsReq>, claims: Claims) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();
    match users
        .filter(user_id.eq(Uuid::parse_str(&*claims.user_id).unwrap()))
        .filter(active.eq(true))
        .select(User::as_select())
        .first(connection)
    {
        Ok(user) => {
            if remove_funds_req.funds > 0f64 && user.credit >= remove_funds_req.funds {
                match diesel::update(users)
                    .filter(user_id.eq(user.user_id))
                    .filter(active.eq(true))
                    .set(credit.eq(credit - remove_funds_req.funds))
                    .execute(connection)
                {
                    Ok(_) => return json!({"status": "ok", "message": "Funds removed".to_string()}),
                    Err(_) => {
                        return json!({"status": "error", "message": "Failed to remove funds".to_string()})
                    }
                }
            }
            json!({"status": "error", "message": "Insufficient funds".to_string()})
        }
        Err(_) => {
            json!({"status": "error", "message": "User not found".to_string()})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddFundsReq {
    funds: f64,
}

#[post("/add_funds", format = "application/json", data = "<add_funds_req>")]
pub fn add_funds(add_funds_req: Json<AddFundsReq>, claims: Claims) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    if add_funds_req.funds > 0f64 {
        match diesel::update(users)
            .filter(user_id.eq(Uuid::parse_str(&*claims.user_id).unwrap()))
            .filter(active.eq(true))
            .set(credit.eq(credit + add_funds_req.funds))
            .execute(connection)
        {
            Ok(_) => return json!({"status": "ok", "message": "Funds added".to_string()}),
            Err(_) => return json!({"status": "error", "message": "User not found".to_string()}),
        }
    }

    json!({"status": "error", "message": "Invalid request"})
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
pub fn user_details(claims: Claims) -> Value {
    use crate::schema::open_em::profiles::dsl::*;
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    let mut data = UserDetails {
        email: "".to_string(),
        credit: 0.0,
        first_name: "".to_string(),
        last_name: "".to_string(),
    };

    match users
        .filter(user_id.eq(Uuid::parse_str(&*claims.user_id).unwrap()))
        .filter(active.eq(true))
        .select(User::as_select())
        .first(connection)
    {
        Ok(user) => {
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
                    json!({"status": "ok",
                        "message": "User details successfully retrieved".to_string(),
                        "data": data})
                }
                Err(_) => {
                    json!({"status": "ok", "message":
                        "No matching user profile".to_string(),
                        "data": data
                    })
                }
            }
        }
        Err(_) => {
            json!({"status": "ok",
                "message":"User not found".to_string(),
                "data": data
            })
        }
    }
}

#[post("/remove_account")]
pub fn remove_account(claims: Claims) -> Value {
    use crate::schema::open_em::users::dsl::*;

    let connection = &mut establish_connection();

    match diesel::update(users)
        .filter(user_id.eq(Uuid::parse_str(&*claims.user_id).unwrap()))
        .set(active.eq(false))
        .execute(connection)
    {
        Ok(_) => {
            json!({"status": "ok", "message": "Account successfully deleted".to_string()})
        }
        Err(_) => {
            json!({"status": "error", "message": "Failed to remove account".to_string()})
        }
    }
}
