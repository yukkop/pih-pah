use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;

const HASH_LENGTH: usize = 25;

pub enum Language {
    Ru,
    En,
}

lazy_static::lazy_static! {
    pub static ref LANGUAGE: Language = Language::En;
}

pub enum Uniq {
    Module(&'static str),
    Id(&'static str),
}

pub fn trans(text: Arc<String>, uniq: Uniq) -> String {
    let _id = match uniq {
        Uniq::Module(module) => hash_string(text.as_str(), module, HASH_LENGTH),
        Uniq::Id(id) => id.to_string(),
    };

    // TODO get from hashmap translated data or translate by internet resources

    text.to_string()
}

fn hash_string(input: &str, key: &str, hash_length: usize) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("help me ples");
    mac.update(input.as_bytes());
    let result = mac.finalize().into_bytes().to_vec();

    let truncated = &result[..hash_length.min(result.len())];
    hex::encode(truncated)
}