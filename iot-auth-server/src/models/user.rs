use bson;
use chrono::Utc;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

use super::lib::mongo::DBWrapper;

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
    let db = DBWrapper::new();
    if db.is_err() {
      // TODO: Handle this in a better way than saying "got error"
      return Err(String::from(
        "[User::create()] Got error initializing DBWrapper",
      ));
    }
    // await connection
    let client = db.unwrap().establish_connection().await;
    // if we can't connect, throw an error
    if client.is_err() {
      return Err(String::from(
        "[User::create()] Got error connecting to requested database",
      ));
    }

    let collection = client.unwrap().collection("users");
    let res = collection.insert_one(self.to_doc().clone(), None).await;
    if res.is_ok() {
      let returned = collection.find_one(self.to_doc(), None).await;

      if returned.is_ok() {
        Ok(returned.unwrap())
      } else {
        Err(String::from(
          "[User::create()] Got error at collection.find_one(<doc>)",
        ))
      }
    } else {
      println!("Error handler at User::create {:?}", res.unwrap_err());
      Err(String::from(
        "[User::create()] Got error inserting new user doc",
      ))
    }
  }
}
