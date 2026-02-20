pub struct LinkCheckResult {
    pub url: String,
    pub title: Option<String>,
    pub http_status: Option<u16>,
    pub error: Option<String>,
}

impl LinkCheckResult {
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            http_status: None,
            error: None,
        }
    }
    pub fn is_valid_link(&self) -> bool {
        self.http_status == Some(200) && self.error.is_none()
    }

    pub fn produce_link_checker_report(&self) -> String {
        let label = self.title.as_deref().unwrap_or(&self.url);

        if self.is_valid_link() {
            format!("[OK] {} -> {}", label, self.url)
        } else {
            let reason = match (self.http_status, &self.error) {
                (_, Some(err)) => err.clone(),
                (Some(status), _) => format!("HTTP status error: {}", status),
                (None, None) => "Unknown error".to_string(),
            };
            format!("[FAIL] {} -> {} (Reason: {})", label, self.url, reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_link_should_return_true_when_status_is_ok() {
        let mut result = LinkCheckResult::new("https://google.com".to_string());
        result.http_status = Some(200);
        assert!(result.is_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_network_fails() {
        let mut result = LinkCheckResult::new("https://fake_domain.com".to_string());
        result.error = Some("Connection refused".to_string());
        assert!(!result.is_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_status_is_not_found() {
        let mut result = LinkCheckResult::new("https://example.com/404".to_string());
        result.http_status = Some(404);
        assert!(!result.is_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_status_is_server_error() {
        let mut result = LinkCheckResult::new("https://site.com/500".to_string());
        result.http_status = Some(500);
        assert!(!result.is_valid_link());
    }

    #[test]
    fn format_report_should_use_title_when_available() {
        let mut result = LinkCheckResult::new("https://rust-lang.org".to_string());
        result.title = Some("Rust Programming Language".to_string());
        result.http_status = Some(200);
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
        result.error = Some("Timeout".to_string());
        let report = result.produce_link_checker_report();
        assert!(report.starts_with("[FAIL]"));
        assert!(report.contains("Reason: Timeout"));
    }
}
