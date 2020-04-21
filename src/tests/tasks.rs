use rocket::http::{Status, ContentType};
use rocket::local::{Client, LocalResponse};
use diesel::prelude::*;
use serde_json;
use parking_lot::Mutex;
use dotenv::dotenv;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::models::{OptionalizedTask, Task, Priority};
use crate::db::DbConnection;
use crate::db::schema::tasks;

const TASKS_ROOT: &'static str = "/api/v0/tasks";

fn sample_op_task() -> OptionalizedTask {
  return OptionalizedTask {
    id: Some(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()),
    title: Some(String::from("(Test) title")),
    details: Some(String::from("(Test) details")),
    created_timestamp: Some(NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)),
    updated_timestamp: None,
    deadline: Some(NaiveDate::from_ymd(2020, 7, 8).and_hms(9, 10, 11)),
    completed_timestamp: None,
    priority: Some(Priority::High),
    persistent: Some(true),
    supertask: None,
  };
}

fn panic_on_http_error(res: &mut LocalResponse, panic: bool) -> serde_json::Value {
  // get the response error message
  let body = res.body_string().expect("failed to get response JSON");
  let error: serde_json::Value = serde_json::from_str(&body).expect("failed to parse JSON into error");
  // then panic with message
  if panic {
    panic!("failed to create task: {}", error["error"])
  }
  return error;
}

fn json_to_task(res: &mut LocalResponse) -> Task {
  // get the response error message
  let body = res.body_string().expect("failed to get response JSON");
  let task: Task = serde_json::from_str(&body).expect("failed to parse JSON into task");
  return task;
} 

// https://github.com/SergioBenitez/Rocket/blob/master/examples/todo/src/tests.rs
// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! assert_response_task {
  (mut $op_task:ident, &mut $res:ident) => {
    let task = json_to_task(&mut $res);
    $op_task.updated_timestamp = task.updated_timestamp;
    assert_eq!($op_task, task, "response task should be same as sample");
  }
}

macro_rules! test_task {
  (|$client:ident, $conn:ident, $op_task:ident| $block:expr) => ({
    #![allow(unused_mut)]
    dotenv().ok();
    // lock to prevent db collision
    let _lock = DB_LOCK.lock();
    // set up rocket
    let rocket = crate::rocket();
    // set up db connection
    let db = DbConnection::get_one(&rocket);
    // construct macro variables
    let $client = Client::new(rocket).expect("failed to instantiate Rocket client");
    let $conn = db.expect("failed to get database connection for testing");

    let mut $op_task = sample_op_task();

    // insert sample task into db
    let mut res = create(&$client, &$op_task);
    // should get 200
    if (res.status() != Status::Ok) {
      panic_on_http_error(&mut res, true);
    }
    // check response task
    assert_response_task!(mut $op_task, &mut res);
    
    $block;

    // delete sample task from db
    let mut res = delete(&$client, &$op_task);
    if (res.status() == Status::Ok) {
      assert_response_task!(mut $op_task, &mut res);
    } else {
      // get the response error message
      let http_error = panic_on_http_error(&mut res, false);
      // manually delete task through db connections
      let id = $op_task.id.expect("failed to get task id");
      let task = diesel::delete(
        tasks::table.filter(tasks::columns::id.eq(id))
      ).get_result::<Task>(&*$conn);
      match task {
        Ok(task) => {
          $op_task.updated_timestamp = task.updated_timestamp;
          assert_eq!($op_task, task, "response task should be same as sample");
        },
        Err(e) => panic!(
          "failed to delete task both by http request and db connection,\
          \nhttp response: {},\
          \ndiesel error: {}\n",
          http_error["error"],
          &e.to_string(),
        )
      }
      // and panic
      panic!(
        "failed to delete task: {},\
        \nbut successfully deleted through manual diesel connection", 
        http_error["error"],
      );
    }
  })
}

fn read<'a>(client: &'a Client, op_task: &OptionalizedTask) -> LocalResponse<'a> {
  let id = op_task.id.expect("failed to get task id");
  let path = format!("{}/?id={}", TASKS_ROOT, &id);
  return client.get(path).dispatch();
}

fn create<'a>(client: &'a Client, op_task: &OptionalizedTask) -> LocalResponse<'a> {
  return client.post(TASKS_ROOT)
    .header(ContentType::JSON)
    .body(serde_json::to_string(&op_task).unwrap())
    .dispatch();  
}

fn update<'a>(client: &'a Client, op_task: &OptionalizedTask) -> LocalResponse<'a> {
  let path = format!("{}/?id={}", TASKS_ROOT, op_task.id.unwrap());
  return client.put(path)
    .header(ContentType::JSON)
    .body(serde_json::to_string(&op_task).unwrap())
    .dispatch();
}

fn delete<'a>(client: &'a Client, op_task: &OptionalizedTask) -> LocalResponse<'a> {
  let path = format!("{}/?id={}", TASKS_ROOT, op_task.id.unwrap());
  return client.delete(path).dispatch();
}

#[test]
fn test_read_all() {
  test_task!(|client, conn, op_task| {
    let res = client.get(TASKS_ROOT).dispatch();
    // should receive 200
    assert_eq!(res.status(), Status::Ok);
  });
}

#[test]
fn test_create_and_delete() {
  // just test setup, create, delete 
  test_task!(|client, conn, op_task| {});
}

#[test]
fn test_read() {
  test_task!(|client, conn, op_task| {
    // send READ request
    let mut res = read(&client, &op_task);
    // should receive 200
    if res.status() != Status::Ok {
      panic_on_http_error(&mut res, true);
    }
    assert_response_task!(mut op_task, &mut res);
  });
}

#[test]
fn test_update() {
  test_task!(|client, conn, op_task| {
    // make some change to task
    op_task.priority = Some(Priority::Medium);
    op_task.details = Some(String::from("(Test) changed details"));
    op_task.completed_timestamp = Some(NaiveDate::from_ymd(2019, 12, 12).and_hms(9, 10, 11));
    // send PUT request
    let mut res = update(&client, &op_task);
    // should receive 200
    if res.status() != Status::Ok {
      panic_on_http_error(&mut res, true);
    }
    assert_response_task!(mut op_task, &mut res);
  });
}