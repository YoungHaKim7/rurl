use clap::Parser;
use http::send_request;
use reqwest::Method;
use std::{fs, io::Result};

use stdout::format_http_response;
use utils::{can_send_body, is_supported_http_method, is_valid_url, str_to_reqwest_method};

use crate::args::CursalArgs;

mod args;
mod http;
mod stdout;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let args = CursalArgs::parse();

    if !is_valid_url(args.url.as_str()) {
        print_error!("The URL is not in the correct format.");
    }

    let method = match args.method {
        Some(method) => str_to_reqwest_method(method.as_str()),
        None => Method::GET,
    };

    if !is_supported_http_method(&method) {
        print_error!("Invalid HTTP Method.");
    }

    let mut body = match args.data {
        Some(data) => {
            if !can_send_body(&method) {
                print_error!("Sending a body is not permitted for the HTTP Method");
            }
            data
        }
        None => "".to_string(),
    };

    if body.starts_with("@") {
        let path_to_data_file = &body[1..];
        body = fs::read_to_string(path_to_data_file).expect("Failed to read data");
    }

    let output_message = match send_request(&args.url, &method, &body).await {
        Ok(response) => format_http_response(response).await,
        Err(err) => print_error!(format!("Failed to send http request: {}", err.to_string())),
    };

    match args.output {
        Some(path) => fs::write(path, output_message)
            .expect("Failed to write output message to the given path."),
        None => println!("{}", output_message),
    };

    Ok(())
}
