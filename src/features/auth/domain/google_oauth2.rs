use serde::Deserialize;

use crate::features::{
    auth::{EmailAddress, GoogleID},
    shared::TrimmedString,
};

#[derive(Deserialize)]
pub struct GoogleAccessTokenSuccess {
    pub access_token: TrimmedString<0, 400>,
}

#[derive(Deserialize)]
pub struct GoogleAccessTokenError {
    pub error: String,
    pub error_description: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GoogleAccessTokenResponse {
    Success(GoogleAccessTokenSuccess),
    Error(GoogleAccessTokenError),
}

#[derive(Deserialize)]
pub struct GoogleUserResponse {
    pub sub: GoogleID,
    pub email: EmailAddress,
    pub email_verified: bool,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn google_access_token_response_empty_should_fail_parse() {
        let json = r#"
           {}
           "#;

        let token_response: std::result::Result<GoogleAccessTokenResponse, _> = from_str(json);
        assert!(token_response.is_err());
    }

    #[test]
    fn google_access_token_response_success_should_pass_parse() {
        let json = r#"
            {
                "access_token": "abcd1234"
            }
            "#;

        let token_response: std::result::Result<GoogleAccessTokenResponse, _> = from_str(json);
        assert!(token_response.is_ok());
    }

    #[test]
    fn google_user_invalid_email_should_fail_parse() {
        let json = r#"
           {
               "sub": "user123",
               "email": "invalid-email",
               "email_verified": true
           }
           "#;

        let user: std::result::Result<GoogleUserResponse, _> = from_str(json);
        assert!(user.is_err());
    }

    #[test]
    fn google_user_empty_should_fail_parse() {
        let json = r#"
           {}
           "#;

        let user: std::result::Result<GoogleUserResponse, _> = from_str(json);
        assert!(user.is_err());
    }

    #[test]
    fn google_user_valid_should_pass_parse() {
        let json = r#"
           {
               "sub": "user123",
               "email": "test@gmail.com",
               "email_verified": true
           }
           "#;

        let user: std::result::Result<GoogleUserResponse, _> = from_str(json);
        assert!(user.is_ok());
    }
}
