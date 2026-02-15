use derive_more::{AsRef, Display};
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, AsRef, Display)]
#[as_ref(Uuid)]
pub struct UserID(Uuid);

impl UserID {
    pub fn parse(value: &str) -> Result<Self> {
        if let Ok(id) = Uuid::parse_str(value) {
            Ok(Self(id))
        } else {
            Err(Error::DomainValidationError(vec!["Invalid user id".into()]))
        }
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for UserID {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn invalid_user_id_should_fail_parse() {
        let user_id = UserID::parse("invalid-uuid-string");
        assert!(user_id.is_err());
    }

    #[test]
    fn empty_string_should_fail_parse() {
        let user_id = UserID::parse("");
        assert!(user_id.is_err());
    }

    #[test]
    fn into_inner_should_return_uuid() {
        let uuid = Uuid::new_v4();
        let user_id = UserID::from(uuid);

        assert_eq!(user_id.into_inner(), uuid);
    }

    #[test]
    fn valid_user_id_should_pass_parse() {
        let user_id = UserID::parse("f47ac10b-58cc-4372-a567-0e02b2c3d479");
        assert!(user_id.is_ok());
    }
}
