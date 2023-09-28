use std::sync::Arc;
use ureq::Error;
use entity::req::{ReqLogin, ReqNewUser};

pub fn login(url: &str, account_name: &str, password: &str) -> Result<Arc<String>, Error> {
  let json_body = ReqLogin {
    account_name,
    password, 
  };
  
  let resp = ureq::post(format!("{}/user/login", url).as_str())
    .set("Content-Type", "application/json")
    .send_json(json_body)?;

    let body: entity::res::ResJwtToken = serde_json::from_str(
      resp.into_string().expect("your code is shit").as_str()
    ).expect("your code is shit");
    Ok(body.token.into())
}

pub fn me(url: &str, token: &str) -> Result<entity::res::Me, Error> {
  let resp = ureq::get(format!("{}/user/me", url).as_str())
    .set("content-type", "application/json")
    .set("token", token)
    .call();
    
  match resp {
    Ok(body) => {
      let body: entity::res::Me = serde_json::from_str(
        body.into_string().expect("your code is shit").as_str()
      ).expect("your code is shit");
      Ok(body)
    } 
    Err(err) => Err(err)
  }
}

pub fn register(url: &str, name: &str, account_name: &str, password: &str) -> Result<entity::res::Me, Error> {
  let resp = ureq::get(format!("{}/user/register", url).as_str())
    .set("content-type", "application/json")
    .call();

  let json_body = ReqNewUser {
    name,
    account_name,
    password, 
    language_id: 1,
  };
    
  match resp {
    Ok(body) => {
      let body: entity::res::Me = serde_json::from_str(
        body.into_string().expect("your code is shit").as_str()
      ).expect("your code is shit");
      Ok(body)
    } 
    Err(err) => Err(err)
  }
}
