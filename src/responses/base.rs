use rocket_contrib::json::JsonValue;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use std::io::Cursor;
use super::ResponseMessage;

pub struct ApiResponse {
  pub data: JsonValue,
  pub status: Status,
}

impl ApiResponse {
  pub fn new(data: JsonValue, status: Status) -> ApiResponse {
    return ApiResponse {
      data,
      status,
    };
  }
}

impl<'r> Responder<'r> for ApiResponse {
  fn respond_to(self, _req: &Request) -> response::Result<'r> {
    return Response::build()
      .status(self.status)
      .sized_body(Cursor::new(self.data.to_string()))
      .header(ContentType::JSON)
      .ok();
  }
}