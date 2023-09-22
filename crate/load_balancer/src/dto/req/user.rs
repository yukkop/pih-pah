use serde::Deserialize;

use crate::model::NewUser;
use crate::tool::get_argon2_config;
use rand::Rng;

#[derive(Deserialize)]
pub struct ReqNewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub account_name: &'a str,
}

impl<'a> From<ReqNewUser<'a>> for NewUser<'a> {
    fn from(req: ReqNewUser<'a>) -> Self {
        let (password_hash, password_salt) = generate_hash(req.password);

        NewUser {
            name: req.name,
            password_hash,
            password_salt,
            account_name: req.account_name,
        }
    }
}

impl<'a> From<&ReqNewUser<'a>> for NewUser<'a> {
    fn from(req: &ReqNewUser<'a>) -> Self {
        let (password_hash, password_salt) = generate_hash(req.password);

        NewUser {
            name: req.name,
            password_hash,
            password_salt,
            account_name: req.account_name,
        }
    }
}

fn generate_hash(password: &str) -> (String, Vec<u8>) {
  let password_salt: Vec<u8> = rand::thread_rng().gen::<[u8; 32]>().to_vec();
  let config = get_argon2_config();
  let password_hash = argon2::hash_encoded(password.as_bytes(), &password_salt, &config).unwrap();
  (password_hash, password_salt)
}

// impl<'a> From<ReqNewUser<'_>> for NewUser<'_> {
//     fn from(req: ReqNewUser) -> Self {
//         let salt: [u8; 16] = rand::thread_rng().gen();
//         let config = get_argon2_config();
//         let hash = argon2::hash_encoded(req.password.as_bytes(), &salt, &config).unwrap();
//
//         let boxed_hash = hash.into_boxed_str();
//         let boxed_salt = salt.to_vec().into_boxed_slice();
//
//         Self {
//           name: req.name,
//           password_hash: hash,
//           password_salt: salt.to_vec(),
//           account_name: req.account_name,
//         }
//     }
// }
