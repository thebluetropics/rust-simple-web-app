use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Unknown error")]
  UnknownError
}

#[derive(Error, Debug)]
pub enum DbError {
  #[error("Unknown database error")]
  UnknownError
}

#[derive(Error, Debug)]
pub enum AuthError {
  #[error("Invalid credentials")]
  InvalidCredentials,

  #[error("Invalid token")]
  InvalidToken,

  #[error("Token expired")]
  TokenExpired,

  #[error("User does exists")]
  UserExists
}
