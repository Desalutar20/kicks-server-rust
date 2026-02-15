use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("Hashed password cannot be empty".into());
        }

        if !(40..=100).contains(&char_count) {
            errors.push("Hashed password must be between 40 and 100 characters".into());
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
        let password = "a".repeat(39);
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn password_too_long_should_fail_parse() {
        let password = "a".repeat(101);
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn empty_password_should_fail_parse() {
        let password = "".to_string();
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn whitespace_only_password_should_fail_parse() {
        let password = "  ".to_string();
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn password_with_valid_length_should_pass_parse() {
        let password = "a".repeat(40);
        assert!(HashedPassword::parse(password).is_ok());

        let password = "a".repeat(100);
        assert!(HashedPassword::parse(password).is_ok());
    }
}
