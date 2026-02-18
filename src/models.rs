#[allow(dead_code)]
pub struct LinkCheckResult {
    pub url: String,
    pub title: Option<String>,
    pub http_status: Option<u16>,
    pub error: Option<String>,
}

#[allow(dead_code)]
impl LinkCheckResult {
    pub fn new(url: String) -> Self {
        Self {
            url,
            title: None,
            http_status: None,
            error: None,
        }
    }

    pub fn is_a_valid_link(&self) -> bool {
        self.http_status == Some(200) && self.error.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_link_should_return_true_when_status_is_ok() {
        let mut result = LinkCheckResult::new("https://google.com".to_string());
        result.http_status = Some(200);
        assert!(result.is_a_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_network_fails() {
        let mut result = LinkCheckResult::new("https://fake_domain.com".to_string());
        result.error = Some("Connection refused".to_string());
        result.http_status = None;
        assert!(!result.is_a_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_status_is_not_found() {
        let mut result = LinkCheckResult::new("https://example_site.com/404".to_string());
        result.http_status = Some(404);
        assert!(!result.is_a_valid_link());
    }

    #[test]
    fn is_valid_link_should_return_false_when_status_is_server_error() {
        let mut result = LinkCheckResult::new("https://site.com/500".to_string());
        result.http_status = Some(500);
        assert!(!result.is_a_valid_link());
    }
}
