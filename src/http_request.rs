use reqwest;

enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

struct HeaderItem {
    key: String,
    value: String,
}

#[tokio::main]
async fn main() {
    let endpoint = "https://catfact.ninja/fact";
    let method = HttpMethod::GET;
    let headers = vec![
        HeaderItem {
            key: String::from("Accept"),
            value: String::from("application/json"),
        },
        HeaderItem {
            key: String::from("Content-Type"),
            value: String::from("application/json"),
        },
    ];

    let response = make_http_request(endpoint, method, headers).await;

    println!("Response: {:?}", response);
    println!("All done!");
}

// TODO: Accept query params
// TODO: Accept body
async fn make_http_request(endpoint: &str, method: HttpMethod, headers: Vec<HeaderItem>) -> String {
    let client = reqwest::Client::new();
    let mut request = match method {
        HttpMethod::GET => client.get(endpoint),
        HttpMethod::POST => client.post(endpoint),
        HttpMethod::PUT => client.put(endpoint),
        HttpMethod::PATCH => client.patch(endpoint),
        HttpMethod::DELETE => client.delete(endpoint),
    };

    for header in headers {
        request = request.header(header.key, header.value);
    }
    let response = request
        .send()
        .await
        .unwrap()
        .text()
        .await;

    match response {
        Ok(response) => response,
        Err(e) => format!("Error: {}", e),
    }
}
