use std::env;

use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Custom,
};
use serde::{Deserialize, Serialize};

use crate::TOKEN_EXPIRATION;

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
