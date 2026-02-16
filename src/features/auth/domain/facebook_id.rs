// use derive_more::{AsRef, Display};
// use unicode_segmentation::UnicodeSegmentation;

// use crate::{Error, Result, features::shared::NonEmptyString};

use crate::features::shared::TrimmedString;

pub type FacebookID = TrimmedString<0, 50>;

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
