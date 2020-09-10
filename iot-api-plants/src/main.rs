#![feature(proc_macro_hygiene, decl_macro)]

pub mod lib;
pub mod models;
pub mod structures;

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;

use structures::plant_data::PlantDataPoint;

#[get("/")]
fn index() -> &'static str {
    "Hello from Rocket!"
}

#[post("/update", format = "application/json", data = "<data>")]
fn update(data: Json<Vec<PlantDataPoint>>) -> rocket::http::Status {
    let plant_data: Vec<PlantDataPoint> = data.into_inner();

    // Don't need to return anything as the nodes/brokers are supposed to be dumb publishers
    rocket::http::Status::Ok
}

fn main() {
    rocket::ignite().mount("/", routes![index, update]).launch();
}
