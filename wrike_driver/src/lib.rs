use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Method};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

// Make the module public so it can be used in `main.rs`
pub mod http_client {
    use super::*;

    // Public async function to send requests
    pub async fn send(
        base_url: &str,
        end_of_url: &str,
        perm_access_token: &str,
        method: &str,
        data: Option<Value>, // For JSON payloads
        additional_headers: HashMap<String, String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Validate access token
        if perm_access_token.is_empty() {
            panic!("Failed to get access token");
        }

        // Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", perm_access_token)).unwrap(),
        );
        
        // Add additional headers
        for (key, value) in additional_headers {
            headers.insert(
                HeaderValue::from_str(&key).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }

        // Set Content-Type if data is JSON
        if data.is_some() {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        // Build the full URL
        let full_url = Url::parse(base_url).unwrap().join(end_of_url).unwrap();

        // Prepare client and request
        let client = Client::new();
        let request_builder = match method.to_uppercase().as_str() {
            "POST" => client.post(full_url),
            "PUT" => client.put(full_url),
            "DELETE" => client.delete(full_url),
            "PATCH" => client.patch(full_url),
            _ => client.get(full_url),
        };

        // Send request
        let response = request_builder
            .headers(headers)
            .json(&data) // Attach JSON data if present
            .send()
            .await?;

        // Log and handle errors (replace with your logging mechanism)
        if !response.status().is_success() {
            eprintln!(
                "Request failed: {} - {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "No response body".to_string())
            );
        }

        Ok(response)
    }
}

