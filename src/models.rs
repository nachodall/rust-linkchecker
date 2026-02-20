use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LinkCheckerError {
    NetworkError(String),
    InvalidUrl(String),
    HttpError(u16),
    IoError(String),
    RunTimeError(String),
}

impl fmt::Display for LinkCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkCheckerError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            LinkCheckerError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            LinkCheckerError::HttpError(status) => write!(f, "HTTP status error: {}", status),
            LinkCheckerError::IoError(msg) => write!(f, "IO error: {}", msg),
            LinkCheckerError::RunTimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl Error for LinkCheckerError {}

/// Enables automatic conversion from tokio::task::JoinError to RunTimeError, allowing seamless error propagation using the '?' operator in main.rs.
impl From<tokio::task::JoinError> for LinkCheckerError {
    fn from(err: tokio::task::JoinError) -> Self {
        LinkCheckerError::RunTimeError(err.to_string())
    }
}

pub struct LinkCheckResult {
    pub url: String,
    pub title: Option<String>,
    pub status: Result<u16, LinkCheckerError>,
}

impl LinkCheckResult {
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            status: Ok(0), // Initial state, will be updated
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self.status, Ok(200))
    }

    pub fn produce_link_checker_report(&self) -> String {
        let label = self.title.as_deref().unwrap_or(&self.url);

        match &self.status {
            Ok(200) => format!("[OK] {} -> {}", label, self.url),
            Ok(code) => format!(
                "[FAIL] {} -> {} (Reason: HTTP status error: {})",
                label, self.url, code
            ),
            Err(err) => format!("[FAIL] {} -> {} (Reason: {})", label, self.url, err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ok_should_return_true_when_status_is_200() {
        let mut result = LinkCheckResult::new("https://google.com".to_string());
        result.status = Ok(200);
        assert!(result.is_ok());
    }

    #[test]
    fn is_ok_should_return_false_when_network_fails() {
        let mut result = LinkCheckResult::new("https://fake_domain.com".to_string());
        result.status = Err(LinkCheckerError::NetworkError(
            "Connection refused".to_string(),
        ));
        assert!(!result.is_ok());
    }

    #[test]
    fn is_ok_should_return_false_when_status_is_not_found() {
        let mut result = LinkCheckResult::new("https://example.com/404".to_string());
        result.status = Ok(404);
        assert!(!result.is_ok());
    }

    #[test]
    fn is_ok_should_return_false_when_status_is_server_error() {
        let mut result = LinkCheckResult::new("https://site.com/500".to_string());
        result.status = Ok(500);
        assert!(!result.is_ok());
    }

    #[test]
    fn format_report_should_use_title_when_available() {
        let mut result = LinkCheckResult::new("https://rust-lang.org".to_string());
        result.title = Some("Rust Programming Language".to_string());
        result.status = Ok(200);
        let report = result.produce_link_checker_report();
        assert!(report.contains("Rust Programming Language"));
        assert!(report.starts_with("[OK]"));
    }

    #[test]
    fn format_report_should_use_url_when_title_is_missing() {
        let result = LinkCheckResult::new("https://www.421.news/es/".to_string());
        let report = result.produce_link_checker_report();
        assert!(report.contains("https://www.421.news/es/"));
    }

    #[test]
    fn format_report_should_show_error_message_on_failure() {
        let mut result = LinkCheckResult::new("https://the_timeout_error_page.com".to_string());
        result.status = Err(LinkCheckerError::NetworkError("Timeout".to_string()));
        let report = result.produce_link_checker_report();
        assert!(report.starts_with("[FAIL]"));
        assert!(report.contains("Reason: Network error: Timeout"));
    }
}
