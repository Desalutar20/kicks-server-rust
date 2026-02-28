use crate::features::shared::TrimmedString;

pub const GET_ALL_USERS_SEARCH_MAX_LENGTH: usize = 100;

pub type GetAllUsersSearch = TrimmedString<1, GET_ALL_USERS_SEARCH_MAX_LENGTH>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_all_users_search_too_long_should_fail_parse() {
        let search = "a".repeat(GET_ALL_USERS_SEARCH_MAX_LENGTH + 1);
        assert!(GetAllUsersSearch::parse(search).is_err())
    }

    #[test]
    fn empty_get_all_users_search_should_fail_parse() {
        let search = "".into();
        assert!(GetAllUsersSearch::parse(search).is_err());
    }

    #[test]
    fn whitespace_only_get_all_users_search_should_fail_parse() {
        let search = "  ".into();
        assert!(GetAllUsersSearch::parse(search).is_err());
    }

    #[test]
    fn valid_get_all_users_search_should_pass_parse() {
        let search = "search".into();
        assert!(GetAllUsersSearch::parse(search).is_ok());
    }
}
