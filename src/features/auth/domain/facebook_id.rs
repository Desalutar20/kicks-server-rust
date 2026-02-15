use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct FacebookID(String);

impl FacebookID {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors: Vec<String> = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("Invalid facebook id".into());
        }

        if char_count > 50 {
            errors.push("FacebookID must be less than or equal to 50 characters.".into());
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
    fn facebook_id_too_long_should_fail_parse() {
        let id = "a".repeat(51);
        assert!(FacebookID::parse(id).is_err())
    }

    #[test]
    fn empty_facebook_id_should_fail_parse() {
        let id = "".into();
        assert!(FacebookID::parse(id).is_err());
    }

    #[test]
    fn whitespace_only_facebook_id_should_fail_parse() {
        let id = "  ".into();
        assert!(FacebookID::parse(id).is_err());
    }

    #[test]
    fn valid_facebook_id_should_pass_parse() {
        let id = "facebookid".into();
        assert!(FacebookID::parse(id).is_ok());
    }
}
