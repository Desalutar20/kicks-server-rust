mod app_user;
mod non_empty_string;
mod trimmed_string;

pub use app_user::*;
pub use non_empty_string::*;
pub use trimmed_string::*;
use unicode_segmentation::UnicodeSegmentation;

use crate::{Error, Result};

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
