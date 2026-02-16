use crate::features::shared::TrimmedString;

pub type OAuth2Code = TrimmedString<0, 100>;
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn code_too_long_should_fail_parse() {
        let id = "a".repeat(101);
        assert!(OAuth2Code::parse(id).is_err())
    }

    #[test]
    fn empty_code_should_fail_parse() {
        let id = "".into();
        assert!(OAuth2Code::parse(id).is_err());
    }

    #[test]
    fn whitespace_only_code_should_fail_parse() {
        let id = "  ".into();
        assert!(OAuth2Code::parse(id).is_err());
    }

    #[test]
    fn valid_code_should_pass_parse() {
        let id = "OAuth2Code".into();
        assert!(OAuth2Code::parse(id).is_ok());
    }
}
