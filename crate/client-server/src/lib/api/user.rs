use std::sync::Arc;
use crate::lib::api::Error;
use entity::req::{ReqLogin, ReqNewUser};
use ureq::OrAnyStatus;
  

pub fn login(url: &str, account_name: &str, password: &str) -> Result<Arc<String>, Error> {
  let json_body = ReqLogin {
    account_name,
    password, 
  };

  let resp = ureq::post(format!("{}/user/login", url).as_str())
    .set("Content-Type", "application/json")
    .send_json(json_body)
    .or_any_status();
   
  match resp {
    Ok(body) => {
      if body.status() >= 400 {
        return Err( Error::from_string(body.into_string().unwrap_or_else(|_| "body is empty".to_string())))
      }

      let body: entity::res::ResJwtToken = serde_json::from_str(
        body.into_string().expect("your code is shit").as_str()
      ).expect("your code is shit");
      Ok(body.token.into())
    },
    Err(err) => panic!("transport error: {}", err)
  }
}

pub fn me(url: &str, token: &str) -> Result<entity::res::Me, Error> {
  let resp = ureq::get(format!("{}/user/me", url).as_str())
    .set("content-type", "application/json")
    .set("token", token)
    .call()
    .or_any_status();
    
  match resp {
    Ok(body) => {
      if body.status() >= 400 {
        return Err( Error::from_string(body.into_string().unwrap_or_else(|_| "body is empty".to_string())))
      }

      let body: entity::res::Me = serde_json::from_str(
        body.into_string().expect("your code is shit").as_str()
      ).expect("your code is shit");
      Ok(body)
    },
    Err(err) => panic!("transport error: {}", err)
  }
}

pub fn register(url: &str, name: &str, account_name: &str, password: &str) -> Result<entity::res::Me, Error> {
  let json_body = ReqNewUser {
    name,
    account_name,
    password, 
    language_id: 1,
  };

  let resp = ureq::post(format!("{}user/register", url).as_str())
    .set("content-type", "application/json")
    .send_json(json_body)
    .or_any_status();
    

  match resp {
    Ok(body) => {
      if body.status() >= 400 {
        return Err( Error::from_string(body.into_string().unwrap_or_else(|_| "body is empty".to_string())))
      }

      let body: entity::res::Me = serde_json::from_str(
        body.into_string().expect("your code is shit").as_str()
      ).expect("your code is shit");
      Ok(body)
    },
    Err(err) => panic!("transport error: {}", err)
  }
}
