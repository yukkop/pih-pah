use rocket::serde::json::serde_json;
use serde::Serialize;

pub fn to_json<T: Serialize>(entity: &T) -> String {
  serde_json::to_string(entity).expect("Failed to serialize user to JSON")
}
