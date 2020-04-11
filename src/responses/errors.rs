use rocket_contrib::json;
use rocket::http::Status;
use super::ApiResponse;
use super::ResponseMessage;

use diesel::result::Error as DieselError;
use std::convert::From;

impl From<DieselError> for ApiResponse {
  fn from(_: DieselError) -> Self {
    return InternalServerError(ResponseMessage::Custom("Diesel Internal Error"));
  }
}

pub fn InternalServerError(message: ResponseMessage) -> ApiResponse {
  let message = match message {    

    ResponseMessage::Custom(m) => m,
    _ => "Internal Server Error",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::InternalServerError,
  };
}

pub fn NotFound(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    _ => "Not Found",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::NotFound,
  };
}

pub fn BadRequest(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    _ => "Bad Request",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::BadRequest,
  };
}

pub fn Unauthorized(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    _ => "Unauthorized",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::Unauthorized,
  };
}

pub fn InvalidUuid() -> ApiResponse {
  return BadRequest(ResponseMessage::Custom("Invalid UUID"));
}

pub fn AlreadyExists(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    _ => "Resource Already Exists",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status {
      code: 409,
      reason: "Conflict",
    },
  };
} 