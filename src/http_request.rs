use reqwest;
use std::collections::HashMap;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

// TODO: Accept query params
// TODO: Accept body
pub async fn make_http_request(
    endpoint: &str,
    method: HttpMethod,
    headers: HashMap<String, String>,
) -> String {
    let client = reqwest::Client::new();
    let mut request = match method {
        HttpMethod::GET => client.get(endpoint),
        HttpMethod::POST => client.post(endpoint),
        HttpMethod::PUT => client.put(endpoint),
        HttpMethod::PATCH => client.patch(endpoint),
        HttpMethod::DELETE => client.delete(endpoint),
    };

    for (key, value) in headers.iter() {
        request = request.header(key, value);
    }

    let response = request.send().await.unwrap().text().await;

    match response {
        Ok(response) => response,
        Err(e) => format!("Error: {}", e),
    }
}
