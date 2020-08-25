#![feature(proc_macro_hygiene, decl_macro)]

pub mod jwt;
pub mod users;

#[macro_use]
extern crate rocket;

use jwt::{generate_token, token_valid};
use rocket::http::RawStr;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json;
use std::default::Default;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
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
        println!("{}", json.unwrap_err());
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

#[get("/validate?<token>")]
fn validate(token: &RawStr) -> Status {
    if token_valid(token) {
        Status::new(200, "Token is valid")
    } else {
        Status::new(403, "Bad token")
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, validate, login])
        .launch();
}
