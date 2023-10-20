use std::str::FromStr;

#[derive(Debug)]
pub struct Error {
  pub message: String,
}

impl FromStr for Error {
  type Err = std::convert::Infallible;

  fn from_str(message: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      message: message.to_string(),
    })
  }
}

impl Error {
  pub fn from_string(message: String) -> Self {
    Self { message }
  }
}

//
// #[derive(Debug)]
// pub enum ApiError {
//   Status({message}),
//   Status({message}),
//   Transport(),
// }
