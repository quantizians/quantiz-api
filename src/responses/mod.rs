use rocket_contrib::json::JsonValue;
use rocket_contrib::json;
use rocket::http::Status;
mod base;
use serde::{Serialize, Deserialize};
pub mod errors;

pub use base::ApiResponse;

pub enum ResponseMessage<'a> {
  Default,
  Custom(&'a str),
}

pub fn Success<'a, E: Serialize + Deserialize<'a>>(entity: &E) -> ApiResponse {
  return ApiResponse {
    data: json!(entity),
    status: Status::Ok,
  }
}

