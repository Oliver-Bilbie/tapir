use reqwest;
use serde_json::value::Value as JsonValue;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

// TODO: Accept query params
pub async fn make_http_request(
    endpoint: String,
    method: HttpMethod,
    headers: HashMap<String, JsonValue>,
    body: HashMap<String, JsonValue>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut request = match method {
        HttpMethod::GET => client.get(endpoint),
        HttpMethod::POST => client.post(endpoint),
        HttpMethod::PUT => client.put(endpoint),
        HttpMethod::PATCH => client.patch(endpoint),
        HttpMethod::DELETE => client.delete(endpoint),
    };

    for (key, value) in headers.iter() {
        request = request.header(key, value.to_string());
    }

    request = request.json(&body);

    Ok(request.send().await?.text().await?)
}
