use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct Password(String);

pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 40;

impl Password {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors: Vec<String> = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("Password cannot be empty".into());
        }

        if !(PASSWORD_MIN_LENGTH..PASSWORD_MAX_LENGTH).contains(&char_count) {
            errors.push(format!(
                "Password must be between {} and {} characters",
                PASSWORD_MIN_LENGTH, PASSWORD_MAX_LENGTH
            ));
        }

        if !errors.is_empty() {
            return Err(Error::DomainValidationError(errors));
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_too_short_should_fail_parse() {
        let password = "a".repeat(PASSWORD_MIN_LENGTH - 1);
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn password_too_long_should_fail_parse() {
        let password = "a".repeat(PASSWORD_MAX_LENGTH);
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn empty_password_should_fail_parse() {
        let password = "".into();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn whitespace_only_password_should_fail_parse() {
        let password = "  ".into();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn valid_password_should_pass_parse() {
        let password = "a".repeat(PASSWORD_MIN_LENGTH);
        assert!(Password::parse(password).is_ok());
    }
}
