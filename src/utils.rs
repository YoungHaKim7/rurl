use reqwest::{Method, Version};
use url::Url;

pub fn can_send_body(method: &Method) -> bool {
    matches!(*method, Method::POST | Method::PUT | Method::PATCH)
    //match *method {
    //    Method::POST | Method::PUT | Method::PATCH => true,
    //    _ => false,
    //}
}

pub fn is_supported_http_method(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
    //match *method {
    //    Method::GET | Method::POST | Method::PUT | Method::PATCH | Method::DELETE => true,
    //    _ => false,
    //}
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
    //match Url::parse(url) {
    //    Ok(_) => true,
    //    Err(_) => false,
    //}
}

pub fn str_to_reqwest_method(method: &str) -> Method {
    match method {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PATCH" => Method::PATCH,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        _ => Method::GET,
    }
}

pub fn reqwest_http_ver_to_str(version: &Version) -> &str {
    match *version {
        Version::HTTP_09 => "0.9",
        Version::HTTP_10 => "1.0",
        Version::HTTP_11 => "1.1",
        Version::HTTP_2 => "2",
        Version::HTTP_3 => "3",
        _ => "Unknown",
    }
}
