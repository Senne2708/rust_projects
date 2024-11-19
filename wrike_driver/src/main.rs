use std::collections::HashMap;
use serde_json::json; // For JSON data construction

use wrike_driver::http_client::send;
#[tokio::main]
async fn main() {
    // Define your base URL and endpoint
    let base_url = "https://api.example.com";
    let endpoint = "/test";

    // Define a dummy access token
    let access_token = "my_access_token";

    // Set the request method (GET, POST, etc.)
    let method = "GET";

    // JSON data to send (can be `None` if not needed)
    let data = Some(json!({ "key": "value" }));

    // Additional headers
    let mut additional_headers = HashMap::new();
    additional_headers.insert("Custom-Header".to_string(), "HeaderValue".to_string());

    // Call the send function and handle the response
    match send(base_url, endpoint, access_token, method, data, additional_headers).await {
        Ok(response) => {
            println!("Response status: {}", response.status());
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);
        }
        Err(e) => eprintln!("Request failed: {}", e),
    }
}

