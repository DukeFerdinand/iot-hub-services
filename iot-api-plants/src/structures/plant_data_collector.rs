use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlantDataCollector {
  /// Local IP, used to allow/disallow connection
  pub ip: String,
  pub common_name: String,
  pub responsible_for_macs: Vec<String>,
}
