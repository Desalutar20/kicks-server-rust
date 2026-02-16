use crate::features::shared::TrimmedString;

pub type HashedPassword = TrimmedString<40, 100>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn password_too_short_should_fail_parse() {
        let password = "a".repeat(39);
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn password_too_long_should_fail_parse() {
        let password = "a".repeat(101);
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn empty_password_should_fail_parse() {
        let password = "".to_string();
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn whitespace_only_password_should_fail_parse() {
        let password = "  ".to_string();
        assert!(HashedPassword::parse(password).is_err());
    }

    #[test]
    fn password_with_valid_length_should_pass_parse() {
        let password = "a".repeat(40);
        assert!(HashedPassword::parse(password).is_ok());

        let password = "a".repeat(100);
        assert!(HashedPassword::parse(password).is_ok());
    }
}
