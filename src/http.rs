use reqwest::{Client, Method, Response, Result};
use url::Url;

pub async fn send_request(url: &String, method: &Method, body: &String) -> Result<Response> {
    let client = Client::new();

    let response = client
        .request(
            method.clone(),
            Url::parse(url).expect("Failed to parse URL"),
        )
        .body(body.clone())
        .send()
        .await?;

    Ok(response)
}
