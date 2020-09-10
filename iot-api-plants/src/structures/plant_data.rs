use serde::{Deserialize, Serialize};

///
/// ### Represents a single data point for a single plant
/// Expect to see a lot of these :)
#[derive(Debug, Serialize, Deserialize)]
pub struct PlantDataPoint {
  /// Plant name attached to monitor, in C
  pub name: String,
  /// Bluetooth address of monitor
  pub mac_addr: String,
  /// Measured in lux
  pub temperature: f32,
  /// Percentage?
  pub moisture: u32,
  pub light_level: u32,
  pub fertilizer: u16,
  pub battery: u16,
}
