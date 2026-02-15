use derive_more::{AsRef, Display};
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct GoogleID(String);

impl GoogleID {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors = Vec::new();

        value.retain(|c| !c.is_whitespace());
        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            return Err(Error::Conflict("Invalid google id".into()));
        }

        if char_count > 50 {
            errors.push("GoogleID must be less than or equal to 50 characters.".into());
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
    fn google_id_too_long_should_fail_parse() {
        let id = "a".repeat(51);
        assert!(GoogleID::parse(id).is_err())
    }

    #[test]
    fn empty_google_id_should_fail_parse() {
        let id = "".into();
        assert!(GoogleID::parse(id).is_err());
    }

    #[test]
    fn whitespace_only_google_id_should_fail_parse() {
        let id = "  ".into();
        assert!(GoogleID::parse(id).is_err());
    }

    #[test]
    fn valid_google_id_should_pass_parse() {
        let id = "googleid".into();
        assert!(GoogleID::parse(id).is_ok());
    }
}
