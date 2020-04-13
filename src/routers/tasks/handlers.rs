use rocket;
use uuid::Uuid;
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::db::DbConnection;
use crate::db::schema::tasks;
use crate::models::{Task, NewTask, TaskInfo};
use crate::responses::{
  ResponseMessage, 
  ApiResponse, 
  Success, 
  errors::{
    InvalidUuid,
    InternalServerError,
    NotFound,
    AlreadyExists,
    UnprocessableEntity,
  }
};

#[get("/")]
fn all(connection: DbConnection) -> ApiResponse {
    return match tasks::table.order(tasks::columns::created_date.desc()).load::<Task>(&*connection) {
      Ok(all_tasks) => Success(&all_tasks),
      Err(e) => InternalServerError(ResponseMessage::Custom(&e.to_string())),
    };
}

#[get("/?<id>")]
fn get_by_id(id: String, connection: DbConnection) -> ApiResponse {
  let id = match Uuid::parse_str(&id) {
    Ok(id) => id,
    _ => return InvalidUuid(),
  };
  
  return match tasks::table.find(id).get_result::<Task>(&*connection) {
    Ok(task) => Success(&task),
    _ => NotFound(
      ResponseMessage::Custom(&format!("No task found with id of {}", id))
    ),
  };
}

// FIXME: catch missing field(s)
#[post("/", format="json", data = "<task>")]
fn new(task: Json<NewTask>, connection: DbConnection) -> ApiResponse {
  let task = task.into_inner();
  let task = diesel::insert_into(tasks::table)
    .values(&task)
    .get_result::<Task>(&*connection);
  return match task {
    Ok(task) => Success(&task),
    Err(e) => InternalServerError(ResponseMessage::Custom(&e.to_string())),
  };
}

// use rocket::Request;
// use rocket::http::ContentType;
// #[catch(422)]
// fn unprocessable_entity(req: &Request) -> ApiResponse {
//   if req.content_type() == ContentType::JSON {
    
//   }
// }

#[put("/", format="json", data = "<task>")]
fn update(task: Option<Json<TaskInfo>>, connection: DbConnection) -> ApiResponse {
  let task = match task {
    Some(task) => task,
    _ => return UnprocessableEntity(ResponseMessage::Default),
  };
  let task = task.into_inner();
  let task = diesel::update(tasks::table).set(&task).get_result::<Task>(&*connection);
  return match task {
    Ok(task) => Success(&task),
    Err(e) => InternalServerError(ResponseMessage::Custom(&e.to_string())),
  };
}


pub fn handlers() -> Vec<rocket::Route> {
  return routes![
    all,
    get_by_id,
    new,
    update,
  ];
}
