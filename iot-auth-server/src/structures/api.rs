use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};

use super::auth::token_valid;

#[derive(Debug)]
pub struct ApiKey(String);

#[derive(Debug)]
pub enum ApiKeyError {
  Invalid,
  Expired,
  Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
  type Error = ApiKeyError;

  fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    let keys: Vec<_> = request.headers().get("Authorization").collect();
    match keys.len() {
      0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
      1 if token_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
      1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Expired)),
      _ => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
    }
  }
}

// ==============================
// Tokens
// ==============================

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenResponse {
  token: Option<String>,
  error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JSONResponse<T, E> {
  data: Option<T>,
  error: Option<String>,
  errors: Option<Vec<String>>,
  details: Option<Vec<E>>,
}

impl<T, E> Default for JSONResponse<T, E> {
  fn default() -> Self {
    Self {
      data: None,
      error: None,
      errors: None,
      details: None,
    }
  }
}

impl<T, E> JSONResponse<T, E> {
  fn new(
    data: Option<T>,
    error: Option<String>,
    errors: Option<Vec<String>>,
    details: Option<Vec<E>>,
  ) -> Self {
    Self {
      data,
      error,
      errors,
      details,
    }
  }
}

pub fn send_json_string<T, E>(
  data: Option<T>,
  error: Option<String>,
  errors: Option<Vec<String>>,
  details: Option<Vec<E>>,
) -> String
where
  T: Serialize,
  E: Serialize,
{
  let response: JSONResponse<T, E> = JSONResponse::new(data, error, errors, details);
  let json = serde_json::to_string(&response);
  if json.is_ok() {
    json.unwrap()
  } else {
    let generic_error: JSONResponse<String, String> = JSONResponse::new(
      None,
      Some("Error creating JSON response at /login".to_owned()),
      None,
      None,
    );
    serde_json::to_string(&generic_error).unwrap_or("Problem".to_owned())
  }
}
