use std::time::Duration;

pub struct Pagination;

impl Pagination {
    pub const DEFAULT_PAGE_SIZE: u32 = 1000;
    pub const PAGE_SIZE_OPTIONS: [u32; 3] = [500, 700, 1000];
    pub const ROW_HEIGHT: usize = 36;
}

pub struct QueryParams;

impl QueryParams {
    pub const COLOR: &str = "color";
    pub const END_DATE: &str = "end_date";
    pub const PAGE: &str = "page";
    pub const SEARCH: &str = "search";
    pub const SIZE: &str = "size";
    pub const START_DATE: &str = "start_date";
}

pub const HOME_PAGE: &str = "/pages/dashboard";
pub const MAX_ROUTING_TIME: Duration = Duration::from_secs(1);
