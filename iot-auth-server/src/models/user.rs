use bson;
use chrono::Utc;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use tokio;

use super::lib;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
  pub _id: Option<bson::oid::ObjectId>,
  pub username: String,
  pub email: String,
  // Not optional in the DB sense,
  // but if you're sending around user objects, they will NOT have passwords attached
  pub password: Option<String>,
  pub date_created: Option<i64>,
  pub date_updated: Option<i64>,
}

pub struct NewUser {
  pub username: String,
  pub email: String,
  // Not optional in the DB sense,
  // but if you're sending around user objects, they will NOT have passwords attached
  pub password: Option<String>,
  pub date_created: Option<i64>,
  pub date_updated: Option<i64>,
}

impl User {
  pub fn new(username: String, email: String, password: String) -> Self {
    User {
      _id: None,
      username,
      email,
      password: Some(password),
      date_created: Some(Utc::now().timestamp()),
      date_updated: Some(Utc::now().timestamp()),
    }
  }
  pub fn to_doc(&self) -> Document {
    return doc! {
      "username": &self.username,
      "email": &self.email,
      "password": self.password.as_ref().unwrap_or(&String::from("")),
      "date_created": self.date_updated.unwrap_or(0),
      "date_created": self.date_updated.unwrap_or(0)
    };
  }
  pub async fn create(&self) -> Result<Option<bson::Document>, String> {
    // await connection
    let client = lib::mongo::establish_connection().await;
    if client.is_ok() {
      let collection = client.unwrap().collection("users");

      let res = collection.insert_one(self.to_doc().clone(), None).await;
      if res.is_ok() {
        let doc = res.unwrap();
        let returned = collection.find_one(self.to_doc(), None).await;

        if returned.is_ok() {
          Ok(returned.unwrap())
        } else {
          Err(String::from("Got error at find_one for inserted doc"))
        }
      } else {
        println!("Error handler at User::create {:?}", res.unwrap_err());
        Err(String::from(
          "Got error inserting User into users collection",
        ))
      }
    } else {
      Err(String::from("Got error connecting to DB"))
    }
  }
}
