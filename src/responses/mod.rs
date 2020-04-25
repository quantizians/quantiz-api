#![allow(non_snake_case)]
use rocket::http::Status;
use rocket_contrib::json;
mod base;
use serde::{Deserialize, Serialize};
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
    };
}
