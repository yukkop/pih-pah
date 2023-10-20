use crate::lib::api::Error;
use entity::res::ResServer;
use ureq::OrAnyStatus;

pub fn servers(url: &str, token: &str) -> Result<Vec<ResServer>, Error> {
  let resp = ureq::get(format!("{}/server/get", url).as_str())
    .set("Content-Type", "application/json")
    .set("token", token)
    .call()
    .or_any_status();

  match resp {
    Ok(body) => {
      if body.status() >= 400 {
        return Err(Error::from_string(
          body
            .into_string()
            .unwrap_or_else(|_| "body is empty".to_string()),
        ));
      }

      let body: Vec<entity::res::ResServer> =
        serde_json::from_str(body.into_string().expect("your code is shit").as_str())
          .expect("your code is shit");
      Ok(body)
    }
    Err(err) => panic!("transport error: {}", err),
  }
}
