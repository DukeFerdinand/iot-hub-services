use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::RawStr;
use serde::{Deserialize, Serialize};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  user: String,
  exp: i64,
}

impl Claims {
  /// If a token should always be equal to its representation after serializing and deserializing
  /// again, this function must be used for construction. `DateTime` contains a microsecond field
  /// but JWT timestamps are defined as UNIX timestamps (seconds). This function normalizes the
  /// timestamps.
  pub fn new(user: String) -> Self {
    // normalize the timestamps by stripping of microseconds
    // let iat = iat.date().and_hms_milli(iat.hour(), iat.minute(), iat.second(), 0);
    let exp = Utc::now() + Duration::days(14);
    Self {
      user,
      exp: exp.timestamp(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  secret: String,
}

pub fn generate_token(user: String) -> Result<String, String> {
  let conf = Config {
    secret: "Fake_secret_use_ENV".to_owned(),
  };
  let my_claims = Claims::new(user);

  println!("{:?}", my_claims);

  let token = encode(
    &Header::default(),
    &my_claims,
    &EncodingKey::from_secret(conf.secret.as_ref()),
  );

  if token.is_ok() {
    Ok(token.unwrap())
  } else {
    println!("{}", token.unwrap_err());
    Err("Error generating token!".to_owned())
  }
}

pub fn token_valid(token: &RawStr) -> bool {
  let conf = Config {
    secret: "Fake_secret_use_ENV".to_owned(),
  };
  let result = decode::<Claims>(
    token.as_ref(),
    &DecodingKey::from_secret(conf.secret.as_bytes()),
    &Validation::default(),
  );
  // TODO: Return response for why it's not valid
  result.is_ok()
}
