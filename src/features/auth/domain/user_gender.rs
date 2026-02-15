use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_gender", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserGender {
    Female,
    Male,
    Other,
}

impl UserGender {
    pub fn parse(value: String) -> Result<Self> {
        match value.as_str() {
            v if v.eq_ignore_ascii_case("male") => Ok(UserGender::Male),
            v if v.eq_ignore_ascii_case("female") => Ok(UserGender::Female),
            v if v.eq_ignore_ascii_case("other") => Ok(UserGender::Other),
            _ => Err(Error::DomainValidationError(vec![
                "Must be male, female or other".into(),
            ])),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_user_gender_should_fail_parse() {
        let gender = "".into();
        assert!(UserGender::parse(gender).is_err());
    }

    #[test]
    fn whitespace_only_user_gender_should_fail_parse() {
        let gender = "  ".into();
        assert!(UserGender::parse(gender).is_err());
    }

    #[test]
    fn should_fail_parse() {
        let gender = "invalid gender".into();
        assert!(UserGender::parse(gender).is_err());
    }

    #[test]
    fn valid_user_gender_should_pass_parse() {
        let valid_genders = ["male", "Female", "OTHER"];

        for gender in valid_genders {
            assert!(UserGender::parse(gender.into()).is_ok())
        }
    }
}
