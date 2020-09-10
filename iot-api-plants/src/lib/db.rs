use std;
use std::env;

use mongodb;
use mongodb::{options::ClientOptions, Client, Database};

// const DEFAULT_MONGO_ADDRESS: &'static str = "127.0.0.1";

///
/// ### High level DB controls that don't fit into specific collections
/// Things like
/// - Creating a connection
/// - Authentication
///
pub struct DBWrapper {
  // Not public as we should never use these values outside of methods
  user: String,
  password: String,
  pub database: String,
}

impl DBWrapper {
  /// ### Create a struct that can be used to make an actual connection to database
  /// This just pulls in all needed env vars and validates that the required ones exist
  pub fn new() -> Result<Self, std::env::VarError> {
    // Mount these via docker for dev or include in deploy
    let user = env::var("MONGO_INITDB_ROOT_USERNAME")?;
    let password = env::var("MONGO_INITDB_ROOT_PASSWORD")?;
    Ok(Self {
      user,
      password,
      database: "iot-auth".to_string(),
    })
  }

  /// ### Create database connection to the only database this service needs
  /// Builds on keys retrieved in Self::new, formats connection string, then attempts a database connection
  pub async fn establish_connection(
    &self,
  ) -> Result<std::sync::Arc<Database>, mongodb::error::Error> {
    // Running in docker, so we use a docker host alias (TODO: Put in .env)
    let mut client_options = ClientOptions::parse(&format!(
      "mongodb://{}:{}@mongo-db:27017",
      &self.user, &self.password
    ))
    .await?;
    // Not required, but good for logs
    client_options.app_name = Some("iot-auth".to_string());
    let client = Client::with_options(client_options)?;
    Ok(std::sync::Arc::new(client.database(&self.database)))
  }
}
