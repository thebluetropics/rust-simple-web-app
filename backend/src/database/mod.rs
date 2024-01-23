use anyhow::{bail, Result};
use tokio_postgres::{Client, NoTls};
use std::env;

use crate::error::Error;

pub struct Database {
  pub client: Client
}

pub async fn connect() -> Result<Database> {
  let (client, connection) = match tokio_postgres::connect(
    format!(
      "user={} password={} dbname={} host={} port={}",
      env::var("DB_USER").unwrap(),
      env::var("DB_PASS").unwrap(),
      env::var("DB_NAME").unwrap(),
      env::var("DB_HOST").unwrap(),
      env::var("DB_PORT").unwrap()
    ).as_str(),
    NoTls
  ).await {
    Ok(result) => result,
    Err(_) => bail!(Error::UnknownError)
  };

  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("{}", e);
    }
  });

  Ok(Database { client })
}
