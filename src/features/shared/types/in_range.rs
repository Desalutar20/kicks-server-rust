use derive_more::Display;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
pub struct InRange<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> InRange<MIN, MAX> {
    pub fn parse(value: usize) -> Result<Self> {
        if !(MIN..=MAX).contains(&value) {
            return Err(Error::DomainValidationError(vec![format!(
                "Value must be between {} and {} (inclusive), but got {}",
                MIN, MAX, value
            )]));
        }

        Ok(Self(value))
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn value(&self) -> usize {
        self.0
    }
}
