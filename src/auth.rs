use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};
use rocket::{Outcome, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};

use crate::config::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: i32,
    /// user email
    pub email: String
}

impl Auth {
    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = jwt::EncodingKey::from_secret(secret.as_ref());
        jwt::encode(&jwt::Header::default(), self, &encoding_key).expect("jwt")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    /// Extract auth token from the "Authorization" header
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, Self::Error> {
        let state: State<AppState> = request.guard()?;

        if let Some(auth) = extract_auth_from_request(request, &state.secret) {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_auth_from_request(request: &Request, secret: &[u8]) -> Option<Auth> {
    request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header)
        .and_then(|token| decode_token(token, secret))
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    let token_prefix = "Token ";

    if header.starts_with(token_prefix) {
        Some(&header[token_prefix.len()..])
    } else {
        None
    }
}

fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    let decoding_key = jwt::DecodingKey::from_secret(secret.as_ref());

    jwt::decode(token, &decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}
