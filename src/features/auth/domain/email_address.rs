use derive_more::{AsRef, Display};

use serde::de;
use serde::{Deserialize, Deserializer};
use validator::ValidateEmail;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn parse(value: String) -> Result<Self> {
        if value.validate_email() {
            Ok(Self(value))
        } else {
            Err(Error::DomainValidationError(vec![
                "Invalid email address".into(),
            ]))
        }
    }
}

impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        EmailAddress::parse(s).map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn email_too_long_should_fail_parse() {
        let email = "a".repeat(257);
        assert!(EmailAddress::parse(email).is_err());
    }

    #[test]
    fn empty_email_should_fail_parse() {
        let email = "".into();
        assert!(EmailAddress::parse(email).is_err());
    }

    #[test]
    fn whitespace_only_email_should_fail_parse() {
        let email = "  ".into();
        assert!(EmailAddress::parse(email).is_err());
    }

    #[test]
    fn missing_at_symbol_should_fail_parse() {
        let email = "ursuladomain.com".to_string();
        assert!(EmailAddress::parse(email).is_err());
    }

    #[test]
    fn valid_email_should_pass_parse() {
        let email = "test@gmail.com".into();
        assert!(EmailAddress::parse(email).is_ok());
    }

    #[test]
    fn email_deserialize_invalid_should_fail() {
        let json = r#""invalid-email""#;
        let result: std::result::Result<EmailAddress, _> = from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn email_deserialize_empty_should_fail() {
        let json = r#""""#;
        let result: std::result::Result<EmailAddress, _> = from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn email_deserialize_whitespace_should_fail() {
        let json = r#""   ""#;
        let result: std::result::Result<EmailAddress, _> = from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn email_deserialize_valid_should_pass() {
        let json = r#""test@gmail.com""#;
        let result: std::result::Result<EmailAddress, _> = from_str(json);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), "test@gmail.com");
    }
}
