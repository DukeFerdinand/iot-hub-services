#![feature(proc_macro_hygiene, decl_macro)]

pub mod auth;
pub mod controllers;
pub mod lib;
pub mod models;
pub mod structures;

#[macro_use]
extern crate rocket;

use auth::generate_token;
use futures::executor::block_on;
use rocket::http::Status;
use rocket_contrib::json::Json;
use structures::api::{send_json_string, ApiKey};
use structures::user::{LoginUser, User};

#[get("/")]
fn index() -> &'static str {
    "Only here for https"
}

#[post("/login", format = "application/json", data = "<data>")]
fn login(data: Json<LoginUser>) -> String {
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

#[post("/signup", format = "application/json", data = "<data>")]
fn signup(data: Json<User>) -> String {
    let user = data.into_inner();
    if user.password.is_some() {
        let new_user = models::user::User::new(user.username, user.email, user.password.unwrap());
        let res = block_on(new_user.create());
        match res {
            Ok(doc) => send_json_string::<bson::Document, String>(doc, None, None, None),
            Err(err) => send_json_string::<String, String>(Some(err), None, None, None),
        }
    } else {
        send_json_string::<String, String>(
            None,
            Some(String::from("User must have password")),
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
        .mount("/", routes![index, validate, login, signup])
        .launch();
}
