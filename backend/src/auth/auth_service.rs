use std::ops::Add;
use std::str::FromStr;
use anyhow::{Result, bail};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::offset::Utc;
use chrono::Duration;
use jsonwebtoken::{
  Header,
  Validation,
  errors::ErrorKind as JwtErrorKind
};

use crate::app::App;
use crate::model::user_model;
use crate::error::{AuthError, Error};

#[derive(Serialize, Deserialize)]
struct TokenPayload {
  exp: usize,
  uid: String
}

fn generate_token(app: &App, user_id: &Uuid) -> Result<String> {
  let payload = TokenPayload {
    exp: Utc::now().add(Duration::minutes(5)).timestamp() as usize,
    uid: user_id.to_string()
  };

  match jsonwebtoken::encode(&Header::default(), &payload, app.get_jwt_encode_key()) {
    Ok(token) => Ok(token),
    Err(_) => bail!(Error::UnknownError)
  }
}

pub async fn register(app: &App, user_name: String, password: String) -> Result<String> {
  match user_model::get_by_username(&app.db, &user_name).await {
    Ok(user) => if user.is_some() {
      bail!(AuthError::UserExists);
    },
    Err(_) => bail!(Error::UnknownError)
  }

  let user_id = Uuid::new_v4();
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
    Ok(password_hash) => password_hash.to_string(),
    Err(_) => bail!(Error::UnknownError)
  };
  
  if let Err(_) = user_model::insert(&app.db, &user_id, &user_name, &password_hash).await {
    bail!(Error::UnknownError);
  }
  
  generate_token(app, &user_id)
}

pub async fn login(app: &App, user_name: String, password: String) -> Result<String> {
  let get_user_result = user_model::get_by_username(&app.db, &user_name).await;
  if let Err(_) = get_user_result {
    bail!(Error::UnknownError);
  }

  let user = get_user_result.unwrap();
  if user.is_none() {
    bail!(AuthError::InvalidCredentials);
  }

  let user = user.unwrap();
  let hash = PasswordHash::new(&user.password_hash);
  if let Err(_) = hash {
    bail!(Error::UnknownError);
  }

  if let Err(_) = Argon2::default().verify_password(password.as_bytes(), &hash.unwrap()) {
    bail!(AuthError::InvalidCredentials);
  }

  generate_token(app, &user.id)
}

pub async fn verify_token(app: &App, token: String) -> Result<Uuid> {
  let decode_result = jsonwebtoken::decode::<TokenPayload>(&token, app.get_jwt_decode_key(), &Validation::default());
  
  match decode_result {
    Ok(token_data) => {
      let uuid = Uuid::from_str(&token_data.claims.uid);
      if let Err(_) = uuid {
        bail!(Error::UnknownError)
      }

      Ok(uuid.unwrap())
    },
    Err(e) => {
      match e.kind() {
        JwtErrorKind::InvalidToken => bail!(AuthError::InvalidToken),
        JwtErrorKind::ExpiredSignature => bail!(AuthError::TokenExpired),
        _ => bail!(Error::UnknownError)
      }
    }
  }
}
