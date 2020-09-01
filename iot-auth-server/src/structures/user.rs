use bson;
use serde::{Deserialize, Serialize};

// Don't need all the data on a user object
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
  pub username: String,
  pub email: String,
  // Not optional in the DB sense,
  // but if you're sending around user objects, they will NOT have passwords attached
  pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
  pub _id: Option<bson::oid::ObjectId>,
  pub username: String,
  pub email: String,
  // Not optional in the DB sense,
  // but if you're sending around user objects, they will NOT have passwords attached
  pub password: Option<String>,
  pub date_created: Option<u32>,
  pub date_updated: Option<u32>,
}
