use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

pub const LAST_NAME_MAX_LENGTH: usize = 40;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct LastName(String);

impl LastName {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors: Vec<String> = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("Last name cannot be empty".into());
        }

        if value.chars().any(|c| !c.is_alphabetic()) {
            errors.push("Last name must contain only alphabetic characters".into());
        }

        if char_count > LAST_NAME_MAX_LENGTH {
            errors.push(format!(
                "Last name must be less than or equal to {} characters.",
                { LAST_NAME_MAX_LENGTH },
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
    fn last_name_too_long_should_fail_parse() {
        let last_name = "a".repeat(LAST_NAME_MAX_LENGTH + 1);
        assert!(LastName::parse(last_name).is_err());
    }

    #[test]
    fn empty_last_name_should_fail_parse() {
        let last_name = "".into();
        assert!(LastName::parse(last_name).is_err());
    }

    #[test]
    fn whitespace_only_last_name_should_fail_parse() {
        let last_name = "  ".into();
        assert!(LastName::parse(last_name).is_err());
    }

    #[test]
    fn last_name_with_invalid_symbols_should_fail_parse() {
        let invalid_symbols = ['@', '#', '$', '%', '&', '*', '!', '1', '2', '3', '-', '_'];

        for &symbol in &invalid_symbols {
            let last_name = format!("John{}", symbol);
            assert!(
                LastName::parse(last_name).is_err(),
                "Failed on symbol: {}",
                symbol
            );
        }
    }

    #[test]
    fn valid_last_name_should_pass_parse() {
        let last_name = "John".into();
        assert!(LastName::parse(last_name).is_ok());
    }
}
