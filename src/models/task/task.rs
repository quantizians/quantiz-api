use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::{Queryable, Identifiable, Insertable};
use rocket_contrib::json::JsonValue;
use crate::db::schema::tasks;
use super::Priority;
use crate::responses::ApiEntity;


#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name="tasks"]
pub struct Task {
  pub id: Uuid,
  pub title: String,
  pub details: Option<String>,
  pub created_date: NaiveDateTime,
  pub deadline: Option<NaiveDateTime>,
  pub priority: Option<Priority>,
  pub persistent: Option<bool>,
  pub completed: Option<bool>,
  pub supertask: Option<Uuid>,
}

impl ApiEntity for Task {
  fn to_json_value(&self) -> JsonValue {
    return json!(self);
  }
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
  pub title: &'a str,
  pub details: Option<&'a str>,
  pub deadline: Option< &'a NaiveDateTime>,
  pub priority: Option<&'a Priority>,
  pub persistent: Option<&'a bool>,
  pub completed: Option<&'a bool>,
  pub supertask: Option<&'a Uuid>,
}

pub struct TaskVec(pub Vec<Task>);

impl ApiEntity for TaskVec {
  fn to_json_value(&self) -> JsonValue {
    let TaskVec(tasks) = self;
    return json!(tasks);
  }
}