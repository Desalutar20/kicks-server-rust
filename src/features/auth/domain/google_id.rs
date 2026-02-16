use crate::features::shared::TrimmedString;

pub type GoogleID = TrimmedString<0, 50>;
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn google_id_too_long_should_fail_parse() {
        let id = "a".repeat(51);
        assert!(GoogleID::parse(id).is_err())
    }

    #[test]
    fn empty_google_id_should_fail_parse() {
        let id = "".into();
        assert!(GoogleID::parse(id).is_err());
    }

    #[test]
    fn whitespace_only_google_id_should_fail_parse() {
        let id = "  ".into();
        assert!(GoogleID::parse(id).is_err());
    }

    #[test]
    fn valid_google_id_should_pass_parse() {
        let id = "googleid".into();
        assert!(GoogleID::parse(id).is_ok());
    }
}
