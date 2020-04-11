use rocket_contrib::json::JsonValue;
use rocket::http::Status;
mod base;
pub mod errors;

pub use base::ApiResponse;

pub enum ResponseMessage<'a> {
  Default,
  Custom(&'a str),
}

pub trait ApiEntity {
  fn to_json_value(&self) -> JsonValue;
}

pub fn Success<E: ApiEntity>(entity: E) -> ApiResponse {
  return ApiResponse {
    data: entity.to_json_value(),
    status: Status::Ok,
  }
}

