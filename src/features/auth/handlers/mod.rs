use crate::features::auth::domain::{UserGender, UserRole};
use serde::{Deserialize, Serialize};

mod forgot_password_handler;
mod get_me;
mod logout_handler;
mod oauth2_handler;
mod reset_password_handler;
mod sign_in_handler;
mod sign_up_handler;
mod verify_account_handler;

pub use forgot_password_handler::forgot_password_v1;
pub use get_me::get_me_v1;
pub use logout_handler::logout_v1;
pub use oauth2_handler::{
    facebook_sign_in_v1, get_facebook_redirect_url_v1, get_google_redirect_url_v1,
    google_sign_in_v1,
};
pub use reset_password_handler::reset_password_v1;
pub use sign_in_handler::{generate_session_cookie, sign_in_v1};
pub use sign_up_handler::sign_up_v1;
pub use verify_account_handler::verify_account_v1;

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: UserRole,
    pub gender: Option<UserGender>,
}
