use reqwest::{Client, Method, Response, Result};
use url::Url;

pub async fn send_request(url: &str, method: &Method, body: &str) -> Result<Response> {
    let client = Client::new();

    let response = client
        .request(
            method.clone(),
            Url::parse(url).expect("Failed to parse URL"),
        )
        .body(body.to_owned())
        .send()
        .await?;

    Ok(response)
}
