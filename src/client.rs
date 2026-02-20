use crate::models::{LinkCheckResult, LinkCheckerError};
use crate::parser;
use reqwest::Client;
use std::time::Duration;

pub async fn check_url(client: &Client, result: &mut LinkCheckResult) {
    let url = match reqwest::Url::parse(&result.url) {
        Ok(u) => u,
        Err(_) => {
            result.status = Err(LinkCheckerError::InvalidUrl(result.url.clone()));
            return;
        }
    };

    let response_result = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    match response_result {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                result.status = Ok(status.as_u16());
                if let Ok(body) = response.text().await {
                    result.title = parser::extract_title(&body);
                }
            } else if status.is_client_error() || status.is_server_error() {
                result.status = Err(LinkCheckerError::HttpError(status.as_u16()));
            } else {
                result.status = Ok(status.as_u16());
            }
        }
        Err(e) => {
            result.status = Err(LinkCheckerError::NetworkError(e.to_string()));
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

        assert_eq!(*result.status.as_ref().unwrap(), 200);
        assert!(result.title.is_some());
    }

    #[tokio::test]
    async fn check_url_should_capture_404_error() {
        let client = Client::new();
        let mut result =
            LinkCheckResult::new("https://www.google.com/non-existent-page".to_string());

        check_url(&client, &mut result).await;

        match result.status {
            Err(LinkCheckerError::HttpError(404)) => (),
            _ => panic!("Expected HttpError(404), got {:?}", result.status),
        }
        assert!(!result.is_ok());
    }

    #[tokio::test]
    async fn check_url_should_handle_invalid_domain() {
        let client = Client::new();
        let mut result =
            LinkCheckResult::new("https://this-domain-does-not-exist-12345.com".to_string());

        check_url(&client, &mut result).await;

        assert!(result.status.is_err());
    }
}
