use crate::features::shared::TrimmedString;

pub type ProviderID = TrimmedString<1, 50>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn facebook_id_too_long_should_fail_parse() {
        let id = "a".repeat(51);
        assert!(ProviderID::parse(id).is_err())
    }

    #[test]
    fn empty_facebook_id_should_fail_parse() {
        let id = "".into();
        assert!(ProviderID::parse(id).is_err());
    }

    #[test]
    fn whitespace_only_facebook_id_should_fail_parse() {
        let id = "  ".into();
        assert!(ProviderID::parse(id).is_err());
    }

    #[test]
    fn valid_facebook_id_should_pass_parse() {
        let id = "providerid".into();
        assert!(ProviderID::parse(id).is_ok());
    }
}
