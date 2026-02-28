use crate::features::shared::Pagination;

pub struct Metadata {
    total_count: usize,
    total_pages: usize,
    current_page: usize,
    has_previous: bool,
    has_next: bool,
}

impl Metadata {
    pub fn new<const MIN: usize, const MAX: usize>(
        total_count: usize,
        pagination: Pagination<MIN, MAX>,
    ) -> Self {
        let total_pages = total_count.div_ceil(pagination.limit());

        let has_previous = pagination.page() > 1;
        let has_next = pagination.page() < total_pages;

        Self {
            total_count,
            total_pages,
            has_previous,
            current_page: pagination.page(),
            has_next,
        }
    }

    pub fn into_inner(self) -> (usize, usize, usize, bool, bool) {
        (
            self.total_count,
            self.total_pages,
            self.current_page,
            self.has_previous,
            self.has_next,
        )
    }
}
