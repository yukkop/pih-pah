#[derive(Debug)]
pub struct Error {
  pub message: String,
}

impl Error {
  pub fn from_str(message: &str) -> Self {
    Self {
      message: message.to_string(),
    }
  }

  pub fn from_string(message: String) -> Self {
    Self {
      message,
    }
  }
}
//
// #[derive(Debug)]
// pub enum ApiError {
//   Status({message}),
//   Status({message}),
//   Transport(),
// }
