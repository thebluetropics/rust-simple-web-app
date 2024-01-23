use actix_files::NamedFile;
use actix_web::{web, HttpServer, Responder, HttpResponse, post, get};
use serde::{Deserialize, Serialize};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::result::Result as StdResult;

use crate::app::App;
use crate::auth::auth_service;
use crate::error::AuthError;
use crate::user::user_service;

#[derive(Deserialize)]
struct AuthRequestJson {
  username: String,
  password: String
}

#[derive(Serialize)]
struct TokenResponse {
  token: String
}

impl TokenResponse {
  fn new(token: String) -> TokenResponse {
    TokenResponse { token }
  }
}

#[derive(Serialize)]
struct ProfileResponse {
  username: String,
  display_name: String
}

#[derive(Serialize)]
struct ApiErrorResponse {
  error: String
}

impl ApiErrorResponse {
  fn new(error: &str) -> ApiErrorResponse {
    ApiErrorResponse { error: error.to_string() }
  }
}

#[post("/api/register")]
async fn register(app: web::Data<App>, data: web::Json<AuthRequestJson>) -> impl Responder {
  if data.username.is_empty() || data.password.is_empty() {
    return HttpResponse::BadRequest().finish();
  }

  match auth_service::register(&app, data.username.clone(), data.password.clone()).await {
    Ok(token) => HttpResponse::Ok().json(TokenResponse::new(token)),
    Err(e) => {
      if let Some(&AuthError::UserExists) = e.downcast_ref() {
        return HttpResponse::Conflict().json(ApiErrorResponse::new("user_exists"));
      }

      HttpResponse::InternalServerError().finish()
    }
  }
}

#[post("/api/login")]
async fn login(app: web::Data<App>, data: web::Json<AuthRequestJson>) -> impl Responder {
  if data.username.is_empty() || data.password.is_empty() {
    return HttpResponse::BadRequest().finish();
  }

  match auth_service::login(&app, data.username.clone(), data.password.clone()).await {
    Ok(token) => HttpResponse::Ok().json(TokenResponse::new(token)),
    Err(e) => {
      if let Ok(AuthError::InvalidCredentials) = e.downcast::<AuthError>() {
        return HttpResponse::Unauthorized().json(ApiErrorResponse::new("invalid_credentials"));
      }

      HttpResponse::InternalServerError().finish()
    }
  }
}

#[get("/api/profile")]
async fn get_profile(app: web::Data<App>, auth: BearerAuth) -> impl Responder {
  let verify_result = auth_service::verify_token(&app, auth.token().to_string()).await;

  if let Err(e) = verify_result {
    if let Some(&AuthError::TokenExpired) = e.downcast_ref() {
      return HttpResponse::Unauthorized().json(ApiErrorResponse::new("token_expired"));
    }

    if let Some(&AuthError::InvalidToken) = e.downcast_ref() {
      return HttpResponse::Unauthorized().json(ApiErrorResponse::new("invalid_token"));
    }

    return HttpResponse::InternalServerError().json(ApiErrorResponse::new("unknown_error"));
  }

  let user_id = verify_result.unwrap();
  let profile = user_service::get_user_profile(&app, user_id).await;

  HttpResponse::Ok().json(ProfileResponse {
    username: profile.user_name,
    display_name: profile.display_name
  })
}

async fn index() -> StdResult<NamedFile, std::io::Error> {
  NamedFile::open("../frontend/dist/index.html")
}

pub async fn start(app: web::Data<App>) {
  let server = HttpServer::new(move || {
    actix_web::App::new()
      .app_data(app.clone())
      .service(register)
      .service(login)
      .service(get_profile)
      .service(actix_files::Files::new("/assets", "../frontend/dist/assets"))
      .route("/", web::get().to(index))
      .route("/auth/login", web::get().to(index))
      .route("/auth/register", web::get().to(index))
  })
  .bind(("127.0.0.1", 80))
  .unwrap();

  let _ = server.run().await;
}
