use crate::features::shared::TrimmedString;

pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 40;

pub type Password = TrimmedString<PASSWORD_MIN_LENGTH, PASSWORD_MAX_LENGTH>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_too_short_should_fail_parse() {
        let password = "a".repeat(PASSWORD_MIN_LENGTH - 1);
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn password_too_long_should_fail_parse() {
        let password = "a".repeat(PASSWORD_MAX_LENGTH + 1);
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn empty_password_should_fail_parse() {
        let password = "".into();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn whitespace_only_password_should_fail_parse() {
        let password = "  ".into();
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn valid_password_should_pass_parse() {
        let password = "a".repeat(PASSWORD_MIN_LENGTH);
        assert!(Password::parse(password).is_ok());
    }
}
