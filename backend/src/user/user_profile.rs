pub struct UserProfile {
  pub user_name: String,
  pub display_name: String
}

impl UserProfile {
  pub fn new(user_name: String, display_name: String) -> UserProfile {
    UserProfile {
      user_name,
      display_name
    }
  }
}
