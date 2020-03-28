use rocket::local::Client;
use rocket::http::{Status};

#[test]
fn index() {
  let client = Client::new(crate::rocket()).expect("valid rocket instance");
  let mut response = client.get("/").dispatch();
  assert_eq!(response.status(), Status::Ok);
  assert_eq!(response.body_string(), Some("Quantiz index".into()));
}