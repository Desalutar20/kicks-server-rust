use crate::{Error, Result};
use unicode_segmentation::UnicodeSegmentation;

mod app_user;
mod in_range;
mod metadata;
mod non_empty_string;
mod pagination;
mod trimmed_string;

pub use app_user::*;
pub use in_range::*;
pub use metadata::*;
pub use non_empty_string::*;
pub use pagination::*;
pub use trimmed_string::*;

fn validate_string(value: &str, min: usize, max: usize) -> Result<()> {
    let char_count = value.graphemes(true).count();

    let mut errors = Vec::new();

    if char_count == 0 {
        errors.push("Value cannot be empty.".into());
    }

    if char_count < min {
        errors.push(format!("Value must be at least {} characters.", min));
    }

    if char_count > max {
        errors.push(format!("Value must be at most {} characters.", max));
    }

    if !errors.is_empty() {
        return Err(Error::DomainValidationError(errors));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string_should_fail_parse() {
        let result = validate_string("", 1, 10);
        assert!(result.is_err());
    }

    #[test]
    fn string_too_short_should_fail_parse() {
        let result = validate_string("a", 2, 10);
        assert!(result.is_err());
    }

    #[test]
    fn string_too_long_should_fail_parse() {
        let result = validate_string(&"a".repeat(11), 1, 10);
        assert!(result.is_err());
    }

    #[test]
    fn valid_string_should_pass_parse() {
        let result = validate_string("valid", 1, 10);
        assert!(result.is_ok());
    }
}
