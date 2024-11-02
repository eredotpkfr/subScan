use std::time::Duration;

/// `Subscan` banner module path
pub const SUBSCAN_BANNER_LOG_TARGET: &str = "subscan::banner";
/// `Subscan` environment variable namespace
pub const SUBSCAN_ENV_NAMESPACE: &str = "SUBSCAN";
/// Concurrency level, count of threads
pub const DEFAULT_CONCURRENCY: u64 = 4;
/// Default HTTP timeout as a [`Duration`]
pub const DEFAULT_HTTP_TIMEOUT: Duration = Duration::from_secs(15);
/// Default User-Agent headers for HTTP requests
pub const DEFAULT_USER_AGENT: &str = "\
    Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
    AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/129.0.0.0 Safari/537.36 \
";
/// Asterisk character to indicate all modules
pub const ASTERISK: &str = "*";