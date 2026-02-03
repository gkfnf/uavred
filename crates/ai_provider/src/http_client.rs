//! HTTP Client for AI Providers
//!
//! Provides a unified HTTP client with retry logic, timeout handling,
//! and provider-specific request building.

use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use std::time::Duration;

use crate::types::{ApiError, ProviderConfig};

/// HTTP client wrapper with retry logic
#[derive(Clone)]
pub struct AiHttpClient {
    client: Client,
    config: ProviderConfig,
    retry_policy: RetryPolicy,
}

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub retryable_status_codes: Vec<StatusCode>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            retryable_status_codes: vec![
                StatusCode::TOO_MANY_REQUESTS,
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::BAD_GATEWAY,
                StatusCode::SERVICE_UNAVAILABLE,
                StatusCode::GATEWAY_TIMEOUT,
            ],
        }
    }
}

impl AiHttpClient {
    /// Create a new HTTP client for a provider
    pub fn new(config: &ProviderConfig) -> Self {
        let timeout = Duration::from_secs(config.timeout_seconds);

        let client = Client::builder()
            .timeout(timeout)
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config: config.clone(),
            retry_policy: RetryPolicy::default(),
        }
    }

    /// Create a new client with custom retry policy
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Build a request with provider-specific headers
    fn build_request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.config.endpoint.trim_end_matches('/'), path)
        };

        let mut builder = self.client.request(method, &url);

        // Add authorization header if API key is configured
        if let Some(api_key) = &self.config.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", api_key));
        }

        // Add custom headers
        for (key, value) in &self.config.custom_headers {
            builder = builder.header(key, value);
        }

        // Add default headers
        builder = builder
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");

        // Add organization header if present
        if let Some(org) = &self.config.organization_id {
            builder = builder.header("OpenAI-Organization", org);
        }

        builder
    }

    /// Execute request with retry logic
    async fn execute_with_retry(
        &self,
        build_request: impl Fn() -> RequestBuilder,
    ) -> Result<Response, ApiError> {
        let mut last_error = None;

        for attempt in 0..=self.retry_policy.max_retries {
            let request = build_request();

            match request.send().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        return Ok(response);
                    }

                    // Check if we should retry
                    if attempt < self.retry_policy.max_retries
                        && self.retry_policy.retryable_status_codes.contains(&status)
                    {
                        let delay = self.calculate_delay(attempt);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                        last_error = Some(self.parse_error(status, response).await);
                        continue;
                    }

                    return Err(self.parse_error(status, response).await);
                }
                Err(e) => {
                    if attempt < self.retry_policy.max_retries && self.is_retryable_error(&e) {
                        let delay = self.calculate_delay(attempt);
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                        last_error = Some(ApiError::Network(e.to_string()));
                        continue;
                    }
                    return Err(ApiError::Network(e.to_string()));
                }
            }
        }

        Err(last_error.unwrap_or_else(|| ApiError::Other("Max retries exceeded".to_string())))
    }

    /// Calculate delay with exponential backoff
    fn calculate_delay(&self, attempt: u32) -> u64 {
        let delay = self.retry_policy.base_delay_ms * 2_u64.pow(attempt);
        delay.min(self.retry_policy.max_delay_ms)
    }

    /// Check if an error is retryable
    fn is_retryable_error(&self, error: &reqwest::Error) -> bool {
        error.is_timeout() || error.is_connect() || error.is_request()
    }

    /// Parse error response
    async fn parse_error(&self, status: StatusCode, response: Response) -> ApiError {
        let body = response.text().await.unwrap_or_default();

        match status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                ApiError::Authentication(body)
            }
            StatusCode::TOO_MANY_REQUESTS => ApiError::RateLimit(body),
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => {
                ApiError::InvalidRequest(body)
            }
            StatusCode::NOT_FOUND => ApiError::ModelNotFound(body),
            _ if status.is_server_error() => ApiError::ServerError(body),
            _ => ApiError::Other(format!("HTTP {}: {}", status, body)),
        }
    }

    /// Send GET request
    pub async fn get(&self, path: &str) -> Result<Response, ApiError> {
        self.execute_with_retry(|| self.build_request(Method::GET, path))
            .await
    }

    /// Send POST request with JSON body
    pub async fn post(&self, path: &str, body: impl serde::Serialize) -> Result<Response, ApiError> {
        self.execute_with_retry(|| self.build_request(Method::POST, path).json(&body))
            .await
    }

    /// Send streaming POST request
    pub async fn post_stream(
        &self,
        path: &str,
        body: impl serde::Serialize,
    ) -> Result<reqwest::Response, ApiError> {
        let request = self
            .build_request(Method::POST, path)
            .json(&body)
            .send()
            .await;

        match request {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    Err(self.parse_error(status, response).await)
                }
            }
            Err(e) => Err(ApiError::Network(e.to_string())),
        }
    }

    /// Test connection with latency measurement
    pub async fn test_connection(&self, test_path: &str) -> Result<(bool, u64, Option<String>), ApiError> {
        use std::time::Instant;

        let start = Instant::now();

        match self.get(test_path).await {
            Ok(response) => {
                let latency = start.elapsed().as_millis() as u64;
                let message = response.text().await.ok();
                Ok((true, latency, message))
            }
            Err(ApiError::Authentication(e)) => {
                let latency = start.elapsed().as_millis() as u64;
                Ok((false, latency, Some(e)))
            }
            Err(e) => Err(e),
        }
    }

    /// Simple ping test for latency measurement
    pub async fn ping(&self) -> Result<u64, ApiError> {
        use std::time::Instant;

        let start = Instant::now();

        let response = self
            .client
            .get(&self.config.endpoint)
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(_) | Err(_) => {
                // We accept any response for ping, just measuring round-trip time
                let latency = start.elapsed().as_millis() as u64;
                Ok(latency)
            }
        }
    }
}

/// Utility functions for API requests
pub mod utils {
    use serde_json::Value;

    /// Extract content from OpenAI-compatible response
    pub fn extract_openai_content(response: &Value) -> Option<String> {
        response
            .get("choices")?
            .as_array()?
            .first()?
            .get("message")?
            .get("content")?
            .as_str()
            .map(String::from)
    }

    /// Extract content from streaming chunk
    pub fn extract_streaming_delta(chunk: &Value) -> Option<String> {
        chunk
            .get("choices")?
            .as_array()?
            .first()?
            .get("delta")?
            .get("content")?
            .as_str()
            .map(String::from)
    }

    /// Build OpenAI-compatible chat request body
    pub fn build_openai_request(
        model: &str,
        messages: &[crate::types::ChatMessage],
        stream: bool,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Value {
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": stream,
        });

        if let Some(temp) = temperature {
            body["temperature"] = serde_json::json!(temp);
        }

        if let Some(tokens) = max_tokens {
            body["max_tokens"] = serde_json::json!(tokens);
        }

        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.base_delay_ms, 1000);
    }

    #[test]
    fn test_calculate_delay() {
        let client = AiHttpClient::new(&ProviderConfig::default());
        assert_eq!(client.calculate_delay(0), 1000);
        assert_eq!(client.calculate_delay(1), 2000);
        assert_eq!(client.calculate_delay(2), 4000);
    }
}
