mod handlers;
pub use handlers::handlers;

pub fn root(prefix: &str) -> String {
  return format!("{}/tasks", prefix);
}