#![feature(proc_macro_hygiene, decl_macro)]

pub mod jwt;
pub mod users;

#[macro_use]
extern crate rocket;

use jwt::{generate_token, token_valid};
use rocket::http::Status;
use rocket::request::{Outcome, Request};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json;
use std::default::Default;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug)]
struct ApiKey(String);

#[derive(Debug)]
enum ApiKeyError {
    Invalid,
    Expired,
    Missing,
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if token_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Expired)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TokenResponse {
    token: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct JSONResponse<T, E> {
    data: Option<T>,
    error: Option<String>,
    errors: Option<Vec<String>>,
    details: Option<Vec<E>>,
}

impl<T, E> Default for JSONResponse<T, E> {
    fn default() -> Self {
        Self {
            data: None,
            error: None,
            errors: None,
            details: None,
        }
    }
}

impl<T, E> JSONResponse<T, E> {
    fn new(
        data: Option<T>,
        error: Option<String>,
        errors: Option<Vec<String>>,
        details: Option<Vec<E>>,
    ) -> Self {
        Self {
            data,
            error,
            errors,
            details,
        }
    }
}

fn send_json_string<T, E>(
    data: Option<T>,
    error: Option<String>,
    errors: Option<Vec<String>>,
    details: Option<Vec<E>>,
) -> String
where
    T: Serialize,
    E: Serialize,
{
    let response: JSONResponse<T, E> = JSONResponse::new(data, error, errors, details);
    let json = serde_json::to_string(&response);
    if json.is_ok() {
        json.unwrap()
    } else {
        let generic_error: JSONResponse<String, String> = JSONResponse::new(
            None,
            Some("Error creating JSON response at /login".to_owned()),
            None,
            None,
        );
        serde_json::to_string(&generic_error).unwrap_or("Problem".to_owned())
    }
}

#[get("/")]
fn index() -> Status {
    Status::new(405, "Try /auth-test")
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(data: Json<User>) -> String {
    let user = data.into_inner();
    let token_res = generate_token(user.username);
    if token_res.is_ok() {
        send_json_string::<String, String>(Some(token_res.unwrap()), None, None, None)
    } else {
        send_json_string::<String, String>(
            None,
            Some("Got error at token generation".to_owned()),
            None,
            None,
        )
    }
}

#[get("/validate")]
fn validate(_key: ApiKey) -> Status {
    Status::new(200, "Token is valid")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, validate, login])
        .launch();
}
