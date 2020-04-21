#![allow(non_snake_case)]
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

// use serde_json::Error as SerdeJsonError;
// impl From<SerdeJsonError> for ApiResponse {
//   fn from(error: SerdeJsonError) -> Self {
//     return UnprocessableEntity(
//       ResponseMessage::Custom(&error.to_string())
//     );
//   }
// }

pub fn UnprocessableEntity(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    ResponseMessage::Default => "Unprocessable Entity",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status {
      code: 422,
      reason: "Unprocessable Entity"
    },
  };
}

pub fn InternalServerError(message: ResponseMessage) -> ApiResponse {
  let message = match message {    
    ResponseMessage::Custom(m) => m,
    ResponseMessage::Default => "Internal Server Error",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::InternalServerError, // 500
  };
}

pub fn NotFound(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    ResponseMessage::Default => "Not Found",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::NotFound, // 404
  };
}

pub fn BadRequest(message: ResponseMessage) -> ApiResponse {
  let message = match message {
    ResponseMessage::Custom(m) => m,
    ResponseMessage::Default => "Bad Request",
  };
  return ApiResponse {
    data: json!({"error": message}),
    status: Status::BadRequest, // 400
  };
}

pub fn InvalidUuid() -> ApiResponse {
  return BadRequest(ResponseMessage::Custom("Invalid UUID"));
}

// pub fn AlreadyExists(message: ResponseMessage) -> ApiResponse {
//   let message = match message {
//     ResponseMessage::Custom(m) => m,
//     ResponseMessage::Default => "Resource Already Exists",
//   };
//   return ApiResponse {
//     data: json!({"error": message}),
//     status: Status {
//       code: 409,
//       reason: "Conflict",
//     },
//   };
// } 

// pub fn Unauthenticated(message: ResponseMessage) -> ApiResponse {
//   let message = match message {
//     ResponseMessage::Custom(m) => m,
//     ResponseMessage::Default => "Unauthorized",
//   };
//   return ApiResponse {
//     data: json!({"error": message}),
//     status: Status::Unauthorized, // 401
//   };
// }

// pub fn Unauthorized(message: ResponseMessage) -> ApiResponse {
//   let message = match message {
//     ResponseMessage::Custom(m) => m,
//     ResponseMessage::Default => "Unauthorized",
//   };
//   return ApiResponse {
//     data: json!({"error": message}),
//     status: Status::Forbidden, // 403
//   };
// }