use derive_more::{AsRef, Display};
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

#[cfg(test)]
mod test {
    use super::*;

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
}
