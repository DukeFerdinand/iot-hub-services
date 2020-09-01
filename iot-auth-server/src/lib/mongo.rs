use std;
// use std::env;

use mongodb;
use mongodb::{options::ClientOptions, Client, Database};

// const DEFAULT_MONGO_ADDRESS: &'static str = "127.0.0.1";

pub async fn establish_connection() -> Result<std::sync::Arc<Database>, mongodb::error::Error> {
  // Running in docker, so we use a docker host alias (TODO: Put in .env)
  let mut client_options =
    ClientOptions::parse("mongodb://<user>:<password>@mongo-db:27017").await?;

  client_options.app_name = Some("iot-auth".to_string());

  let client = Client::with_options(client_options)?;

  Ok(std::sync::Arc::new(client.database("iot-auth")))
}
