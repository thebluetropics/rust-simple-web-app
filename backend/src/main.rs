use actix_web::web;
use std::process;

mod app;
use app::App;

mod server;
mod database;
mod model;
mod auth;
mod error;
mod user;

#[tokio::main(flavor="current_thread")]
async fn main() {
  let dotenv_result = dotenv::from_filename(".env");
  if let Err(_) = dotenv_result {
    eprintln!("err: failed to load `.env` file.");
    process::exit(1);
  }
  
  let db_connect_result = database::connect().await;
  if let Err(_) = db_connect_result {
    eprintln!("err: failed to connect to the database.");
    process::exit(1);
  }

  let db = db_connect_result.unwrap();
  let app = web::Data::new(App::new(db));
  
  server::start(app).await;
}
