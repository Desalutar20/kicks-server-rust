use derive_more::{AsRef, Display};
use serde::de;
use serde::{Deserialize, Deserializer};

use crate::Result;
use crate::features::shared::types::validate_string;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, AsRef, Display)]
#[as_ref(str)]
pub struct TrimmedString<const MIN: usize, const MAX: usize>(String);

impl<const MIN: usize, const MAX: usize> TrimmedString<MIN, MAX> {
    pub fn parse(mut value: String) -> Result<Self> {
        value.retain(|c| !c.is_whitespace());

        validate_string(&value, MIN, MAX)?;

        Ok(Self(value))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl<'de, const MIN: usize, const MAX: usize> Deserialize<'de> for TrimmedString<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        TrimmedString::<MIN, MAX>::parse(s).map_err(|e| de::Error::custom(format!("{:?}", e)))
    }
}
