use super::validate_string;
use derive_more::{AsRef, Display};
use serde::de;
use serde::{Deserialize, Deserializer};

use crate::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct NonEmptyString<const MIN: usize, const MAX: usize>(String);

impl<const MIN: usize, const MAX: usize> NonEmptyString<MIN, MAX> {
    pub fn parse(value: String) -> Result<Self> {
        let value = value.trim().to_string();

        validate_string(&value, MIN, MAX)?;

        Ok(Self(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<'de, const MIN: usize, const MAX: usize> Deserialize<'de> for NonEmptyString<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        NonEmptyString::<MIN, MAX>::parse(s).map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}
