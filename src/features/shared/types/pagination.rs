use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pagination<const DEFAULT_LIMIT: usize, const MAX_LIMIT: usize> {
    limit: usize,
    page: usize,
}

impl<const DEFAULT_LIMIT: usize, const MAX_LIMIT: usize> Pagination<DEFAULT_LIMIT, MAX_LIMIT> {
    pub fn parse(limit: Option<usize>, page: Option<usize>) -> Result<Self> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT);
        let page = page.unwrap_or(1);

        if limit == 0 || limit > MAX_LIMIT {
            return Err(Error::DomainValidationError(vec![format!(
                "Limit must be between 1 and {} (inclusive), but got {}.",
                MAX_LIMIT, limit
            )]));
        }

        if page == 0 {
            return Err(Error::DomainValidationError(vec![
                "Page number cannot be zero.".to_string(),
            ]));
        }

        Ok(Pagination { limit, page })
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn offset(&self) -> usize {
        self.page * self.limit - self.limit
    }
}

#[cfg(test)]
mod test {
    use crate::features::shared::Pagination;

    #[test]
    fn zero_limit_should_fail_parse() {
        let limit = Some(0);

        let result = Pagination::<1, 10>::parse(limit, None);
        assert!(result.is_err())
    }

    #[test]
    fn limit_too_big_should_fail_parse() {
        let limit = Some(11);

        let result = Pagination::<1, 10>::parse(limit, None);
        assert!(result.is_err())
    }

    #[test]
    fn zero_page_should_fail_parse() {
        let page = Some(0);

        let result = Pagination::<1, 10>::parse(None, page);
        assert!(result.is_err())
    }

    #[test]
    fn empty_values_should_pass_parse() {
        let result = Pagination::<1, 10>::parse(None, None);
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(1, result.limit());
        assert_eq!(1, result.page());
    }

    #[test]
    fn only_limit_should_use_default_page() {
        let limit = Some(5);
        let page = None;

        let result = Pagination::<1, 10>::parse(limit, page);
        assert!(result.is_ok());

        let pagination = result.unwrap();
        assert_eq!(pagination.limit(), 5);
        assert_eq!(pagination.page(), 1);
    }

    #[test]
    fn only_page_should_use_default_limit() {
        let limit = None;
        let page = Some(3);

        let result = Pagination::<1, 10>::parse(limit, page);
        assert!(result.is_ok());

        let pagination = result.unwrap();
        assert_eq!(pagination.limit(), 1);
        assert_eq!(pagination.page(), 3);
    }

    #[test]
    fn valid_pagination_should_pass_parse() {
        let limit = Some(5);
        let page = Some(2);

        let result = Pagination::<1, 10>::parse(limit, page);
        assert!(result.is_ok());

        let pagination = result.unwrap();
        assert_eq!(pagination.limit(), 5);
        assert_eq!(pagination.page(), 2);
    }
}
