use crate::models::LinkCheckResult;
use crate::parser;
use reqwest::Client;
use std::time::Duration;

pub async fn check_url(client: &Client, result: &mut LinkCheckResult) {
    let response_result = client
        .get(&result.url)
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    match response_result {
        Ok(response) => {
            let status = response.status();
            result.http_status = Some(status.as_u16());

            if status.is_success()
                && let Ok(body) = response.text().await
            {
                result.title = parser::extract_title(&body);
            }
        }
        Err(e) => {
            result.error = Some(e.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::LinkCheckResult;

    #[tokio::test]
    async fn check_url_should_capture_200_and_title() {
        let client = Client::new();
        let mut result = LinkCheckResult::new("https://www.google.com".to_string());

        check_url(&client, &mut result).await;

        assert_eq!(result.http_status, Some(200));
        assert!(result.title.is_some());
        assert!(result.error.is_none());
    }

    #[tokio::test]
    async fn check_url_should_capture_404_error() {
        let client = Client::new();
        let mut result =
            LinkCheckResult::new("https://www.google.com/non-existent-page".to_string());

        check_url(&client, &mut result).await;

        assert_eq!(result.http_status, Some(404));
        assert!(!result.is_valid_link());
    }

    #[tokio::test]
    async fn check_url_should_handle_invalid_domain() {
        let client = Client::new();
        let mut result =
            LinkCheckResult::new("https://this-domain-does-not-exist-12345.com".to_string());

        check_url(&client, &mut result).await;

        assert!(result.error.is_some());
        assert!(result.http_status.is_none());
    }
}
