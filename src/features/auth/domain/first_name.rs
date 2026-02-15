use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

pub const FIRST_NAME_MAX_LENGTH: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct FirstName(String);

impl FirstName {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors: Vec<String> = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("First name cannot be empty".into());
        }

        if value.chars().any(|c| !c.is_alphabetic()) {
            errors.push("First name must contain only alphabetic characters".into());
        }

        if char_count > FIRST_NAME_MAX_LENGTH {
            errors.push(format!(
                "First name must be less than or equal to {} characters.",
                { FIRST_NAME_MAX_LENGTH },
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
    fn first_name_too_long_should_fail_parse() {
        let first_name = "a".repeat(FIRST_NAME_MAX_LENGTH + 1);
        assert!(FirstName::parse(first_name).is_err());
    }

    #[test]
    fn empty_first_name_should_fail_parse() {
        let first_name = "".into();
        assert!(FirstName::parse(first_name).is_err());
    }

    #[test]
    fn whitespace_only_first_name_should_fail_parse() {
        let first_name = "  ".into();
        assert!(FirstName::parse(first_name).is_err());
    }

    #[test]
    fn first_name_with_invalid_symbols_should_fail_parse() {
        let invalid_symbols = ['@', '#', '$', '%', '&', '*', '!', '1', '2', '3', '-', '_'];

        for &symbol in &invalid_symbols {
            let first_name = format!("John{}", symbol);
            assert!(
                FirstName::parse(first_name).is_err(),
                "Failed on symbol: {}",
                symbol
            );
        }
    }

    #[test]
    fn valid_first_name_should_pass_parse() {
        let first_name = "John".into();
        assert!(FirstName::parse(first_name).is_ok());
    }
}
