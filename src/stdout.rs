use reqwest::{Response, StatusCode};

use crate::utils::reqwest_http_ver_to_str;

#[macro_export]
macro_rules! print_error {
    ($message:expr) => {{
        use colored::*;
        use std::process::exit;

        println!("{} {}", "[X]".red(), $message);
        exit(1);
    }};
}

pub async fn format_http_response(response: Response) -> String {
    let reqwest_http_version = response.version();
    let http_version = reqwest_http_ver_to_str(&reqwest_http_version);
    let status_code = response.status().as_u16();
    let http_status =
        StatusCode::from_u16(status_code.clone()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let status_message = http_status.canonical_reason().unwrap_or("Unkown Status");
    let headers = response.headers().clone();
    let content_type = match headers.get("Content-Type") {
        Some(content_type_header) => content_type_header
            .to_str()
            .expect("Failed to convert Content-Type : & HeaderValue to &str"),
        None => "text/plain",
    };
    let content_lenth = response.content_length().unwrap_or_default();

    let text_body = match response.text().await {
        Ok(body) => body,
        Err(err) => print_error!(format!(
            "Failed to parse HTTP response body as text. {}",
            err
        )),
    };

    format!(
        "HTTP/ {} {} {}
Content-Type: {}
Content-Length: {}

{}",
        http_version, status_code, status_message, content_type, content_lenth, text_body
    )
}
