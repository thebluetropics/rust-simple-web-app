use jsonwebtoken::{EncodingKey, DecodingKey};
use std::env;

use crate::database::Database;

pub struct App {
  pub db: Database,
  encode_key: EncodingKey,
  decode_key: DecodingKey
}

impl App {
  pub fn new(db: Database) -> App {
    App {
      db,
      encode_key: EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
      decode_key: DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes())
    }
  }

  pub fn get_jwt_encode_key(&self) -> &EncodingKey {
    &self.encode_key
  }

  pub fn get_jwt_decode_key(&self) -> &DecodingKey {
    &self.decode_key
  }
}
