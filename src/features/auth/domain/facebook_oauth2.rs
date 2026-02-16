use serde::Deserialize;

use crate::features::{
    auth::{EmailAddress, FacebookID},
    shared::TrimmedString,
};

#[derive(Deserialize)]
pub struct FacebookAccessTokenSuccess {
    pub access_token: TrimmedString<0, 400>,
}

#[derive(Deserialize)]
pub struct FacebookAccessTokenError {
    pub error: FacebookErrorDetails,
}

#[derive(Deserialize)]
pub struct FacebookErrorDetails {
    pub message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FacebookAccessTokenResponse {
    Success(FacebookAccessTokenSuccess),
    Error(FacebookAccessTokenError),
}

#[derive(Debug, Deserialize)]
pub struct FacebookUserResponse {
    pub id: FacebookID,
    pub email: EmailAddress,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn facebook_access_token_response_empty_should_fail_parse() {
        let json = r#"
           {}
           "#;

        let token_response: std::result::Result<FacebookAccessTokenSuccess, _> = from_str(json);
        assert!(token_response.is_err());
    }

    #[test]
    fn facebook_user_invalid_email_should_fail() {
        let json = r#"
        {
            "id": "id",
            "email": "invalid-email",
        }
        "#;

        let result: std::result::Result<FacebookUserResponse, _> = from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn facebook_access_token_response_success_should_pass_parse() {
        let json = r#"
            {
                "access_token": "abcd1234"
            }
            "#;

        let token_response: std::result::Result<FacebookAccessTokenResponse, _> = from_str(json);
        assert!(token_response.is_ok());
    }

    #[test]
    fn facebook_user_empty_should_fail_parse() {
        let json = r#"
           {}
           "#;

        let user: std::result::Result<FacebookUserResponse, _> = from_str(json);
        assert!(user.is_err());
    }

    #[test]
    fn facebook_user_valid_should_pass_parse() {
        let json = r#"
           {
               "id": "user123",
               "email": "test@gmail.com"
           }
           "#;

        let user: std::result::Result<FacebookUserResponse, _> = from_str(json);
        println!("{:?}", user);

        assert!(user.is_ok());
    }
}
