use serde::Deserialize;
use validator::Validate;

#[derive(Validate, Debug, Deserialize)]
pub struct RateLimitConfig {
    #[validate(range(min = 4, max = 10))]
    pub sign_up: u32,
    #[validate(range(min = 4, max = 10))]
    pub sign_in: u32,
    #[validate(range(min = 4, max = 10))]
    pub verify_account: u32,
    #[validate(range(min = 4, max = 20))]
    pub get_me: u32,
    #[validate(range(min = 3, max = 5))]
    pub forgot_password: u32,
    #[validate(range(min = 3, max = 5))]
    pub reset_password: u32,
    #[validate(range(min = 3, max = 5))]
    pub logout: u32,
}
