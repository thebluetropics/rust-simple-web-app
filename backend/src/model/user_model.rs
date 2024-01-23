use anyhow::{Result, bail};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::database::Database;
use crate::error::DbError;

pub struct User {
  pub id: Uuid,
  pub name: String,
  pub display_name: Option<String>,
  pub password_hash: String
}

impl User {
  pub fn new(row: &Row) -> User {
    User {
      id: row.get("user_id"),
      name: row.get("user_name"),
      display_name: row.get("user_display_name"),
      password_hash: row.get("user_password_hash")
    }
  }
}

pub async fn get(db: &Database, user_id: &Uuid) -> Result<Option<User>> {
  let sql = "SELECT * FROM users WHERE user_id=$1::UUID;";

  if let Ok(rows) = db.client.query(sql, &[user_id]).await {
    if !rows.is_empty() {
      Ok(Some(User::new(rows.first().unwrap())))
    } else {
      Ok(None)
    }
  } else {
    bail!(DbError::UnknownError);
  }
}

pub async fn get_by_username(db: &Database, user_name: &String) -> Result<Option<User>> {
  let sql = "SELECT * FROM users WHERE user_name=$1::TEXT;";

  if let Ok(rows) = db.client.query(sql, &[user_name]).await {
    if !rows.is_empty() {
      Ok(Some(User::new(rows.first().unwrap())))
    } else {
      Ok(None)
    }
  } else {
    bail!(DbError::UnknownError);
  }
}

pub async fn insert(db: &Database, user_id: &Uuid, user_name: &String, user_password_hash: &String) -> Result<()> {
  let sql = "INSERT INTO users (user_id, user_name, user_password_hash) VALUES ($1::UUID, $2::TEXT, $3::TEXT);";

  if let Err(_) = db.client.query(sql, &[user_id, user_name, user_password_hash]).await {
    bail!(DbError::UnknownError);
  }

  Ok(())
}
