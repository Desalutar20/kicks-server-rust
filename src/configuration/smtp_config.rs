use lettre::transport::smtp::authentication::Credentials;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct SmtpConfig {
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(email, length(min = 1))]
    pub user: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(email, length(min = 1))]
    pub from: String,
}

impl SmtpConfig {
    pub fn credentials(&self) -> Credentials {
        Credentials::new(self.user.to_owned(), self.password.to_owned())
    }
}
