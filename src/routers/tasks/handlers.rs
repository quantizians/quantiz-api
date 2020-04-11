use rocket;
use uuid::Uuid;
use diesel::prelude::*;

use crate::db::DbConnection;
use crate::db::schema::tasks;
use crate::models::{Task, TaskVec, NewTask};
use crate::responses::{
  ResponseMessage, 
  ApiResponse, 
  Success, 
  errors::{
    InvalidUuid,
    InternalServerError,
    NotFound
  }
};

#[get("/")]
fn all(connection: DbConnection) -> ApiResponse {
    return match tasks::table.load::<Task>(&*connection) {
        Ok(all_tasks) => Success(TaskVec(all_tasks)),
        _ => InternalServerError(ResponseMessage::Default),
    };
}

#[get("/?<id>")]
fn get_by_id(id: String, connection: DbConnection) -> ApiResponse {
  let id = match Uuid::parse_str(&id) {
    Ok(id) => id,
    _ => return InvalidUuid(),
  };
  
  return match tasks::table.find(id).get_result::<Task>(&*connection) {
      Ok(task) => Success(task),
      _ => NotFound(
        ResponseMessage::Custom(&format!("No task found with id of {}", id))
      ),
  };
}

// #[post("/tasks", data = "<task>")]
// fn new_task(task: Json<NewTask) {

// }

pub fn handlers() -> Vec<rocket::Route> {
  return routes![
    all,
    get_by_id,
  ];
}