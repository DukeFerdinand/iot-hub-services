#![feature(proc_macro_hygiene, decl_macro)]

pub mod auth;
pub mod structures;

#[macro_use]
extern crate rocket;

use auth::generate_token;
use rocket::http::Status;
use rocket_contrib::json::Json;
use structures::api::{send_json_string, ApiKey};
use structures::user::User;

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
