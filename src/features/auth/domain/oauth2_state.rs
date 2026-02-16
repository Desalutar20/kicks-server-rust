use std::fmt::Display;

use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OAuth2State(Uuid, Option<String>);

const OAUTH2_STATE_DELIMITER: &str = "|";

impl OAuth2State {
    pub fn parse(mut value: String) -> Result<Self> {
        let mut errors = Vec::new();
        value.retain(|c| !c.is_whitespace());

        let char_count = value.graphemes(true).count();

        if value.is_empty() {
            errors.push("OAuth state cannot be empty".into());
        }

        if char_count > 100 {
            errors.push("OAuth state must be less than or equal to 100 characters.".into());
        }

        let splitted: Vec<&str> = value.splitn(2, OAUTH2_STATE_DELIMITER).collect();

        let state_id = Uuid::parse_str(splitted[0]);
        if state_id.is_err() {
            errors.push("Invalid oauth state".into());
        }

        if !errors.is_empty() {
            return Err(Error::DomainValidationError(errors));
        }

        let additional_state = splitted.get(1);

        match additional_state {
            Some(state) => Ok(Self(state_id.unwrap(), Some(state.to_string()))),
            _ => Ok(Self(state_id.unwrap(), None)),
        }
    }

    pub fn into_inner(self) -> (Uuid, Option<String>) {
        (self.0, self.1)
    }
}

impl From<&str> for OAuth2State {
    fn from(value: &str) -> Self {
        let uuid = Uuid::new_v4();
        let trimmed = value.trim();

        if trimmed.graphemes(true).count() > 0 {
            Self(uuid, Some(value.to_owned()))
        } else {
            Self(uuid, None)
        }
    }
}

impl Default for OAuth2State {
    fn default() -> Self {
        Self(Uuid::new_v4(), None)
    }
}

impl Display for OAuth2State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(additional_state) = self.1.as_ref() {
            write!(
                f,
                "{}{}{}",
                self.0, OAUTH2_STATE_DELIMITER, additional_state
            )
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn empty_state_should_fail_parse() {
        let result = OAuth2State::parse("".into());
        assert!(result.is_err());
    }

    #[test]
    fn whitespace_only_state_should_fail_parse() {
        let result = OAuth2State::parse("   ".into());
        assert!(result.is_err());
    }

    #[test]
    fn too_long_state_should_fail_parse() {
        let result = OAuth2State::parse(format!("{}|extra", "a".repeat(101)));
        assert!(result.is_err());
    }

    #[test]
    fn invalid_uuid_should_fail_parse() {
        let result = OAuth2State::parse("not-a-uuid|state".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn valid_state_without_additional_should_pass_parse() {
        let uuid = Uuid::new_v4();
        let result = OAuth2State::parse(uuid.to_string());
        assert!(result.is_ok());

        let state = result.unwrap();
        assert_eq!(state.0, uuid);
        assert!(state.1.is_none());
    }

    #[test]
    fn valid_state_with_additional_should_pass_parse() {
        let uuid = Uuid::new_v4();
        let extra = "extra_state";
        let result = OAuth2State::parse(format!("{}|{}", uuid, extra));
        assert!(result.is_ok());

        let state = result.unwrap();
        assert_eq!(state.0, uuid);
        assert_eq!(state.1.unwrap(), extra.to_string());
    }
}
