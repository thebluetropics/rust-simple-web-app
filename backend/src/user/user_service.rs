use uuid::Uuid;

use crate::{app::App, model::user_model};
use crate::user::UserProfile;

pub async fn get_user_profile(app: &App, user_id: Uuid) -> UserProfile {
  let user = user_model::get(&app.db, &user_id).await.unwrap().unwrap();
  UserProfile::new(user.name, "".to_string())
}
