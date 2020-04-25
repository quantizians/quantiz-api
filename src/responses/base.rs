use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::JsonValue;
use std::io::Cursor;

pub struct ApiResponse {
    pub data: JsonValue,
    pub status: Status,
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
