mod handlers;
pub use handlers::handlers;

pub fn root(prefix: &str) -> String {
  return String::from(format!("{}/tasks", prefix));
}