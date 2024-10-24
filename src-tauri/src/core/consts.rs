pub mod file_processor {
    pub const STATUS: &str = "scanner_status";

    pub const SCAN_COUNTS: &str = "scanner_scan_counts";
    pub const PARSE_COUNTS: &str = "scanner_parse_counts";
    pub const FILTER_COUNTS: &str = "scanner_filter_counts";
}

pub mod deletion {
    pub const CATEGORY_START: &str = "deletion_category_start";
    pub const CATEGORY_COMPLETE: &str = "deletion_category_complete";
}

pub mod status_values {
    pub const SCAN_START: &str = "scan_start";
    pub const PARSE_START: &str = "parse_start";
    pub const FILTER_START: &str = "filter_start";

    pub const SCAN_CANCELLED: &str = "scan_cancelled";
    pub const PARSE_CANCELLED: &str = "parse_cancelled";
    pub const FILTER_CANCELLED: &str = "filter_cancelled";
    pub const DELETION_CANCELLED: &str = "deletion_cancelled";
}
