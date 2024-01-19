use std::fs;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use std::sync::Arc;

use crate::cli::CliOption;
use crate::server::request::HttpRequest;
use crate::server::response::{Body, HttpResponse, StatusCode};

pub fn handle_request(
    request: &HttpRequest,
    options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    match request.path.as_str() {
        "/" => handle_route_root(request, options),
        s if s == "/echo" || s.starts_with("/echo/") => handle_route_echo(request, options),
        s if s == "/user-agent" || s.starts_with("/user-agent/") => {
            handle_route_user_agent(request, options)
        }
        s if s == "/files" || s.starts_with("/files/") => handle_route_files(request, options),
        _ => handle_route_unknown(request, options),
    }
}

fn handle_route_unknown(
    _request: &HttpRequest,
    _options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::not_found())
}

fn handle_route_root(
    _request: &HttpRequest,
    _options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::ok())
}

fn handle_route_echo(
    request: &HttpRequest,
    _options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    let path_parts: Vec<&str> = request.path.split(MAIN_SEPARATOR).collect();
    let body = path_parts[2..].join(MAIN_SEPARATOR_STR);
    let response = HttpResponse::new(StatusCode::Ok, Body::Text(body));
    Ok(response)
}

fn handle_route_user_agent(
    request: &HttpRequest,
    _options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    let user_agent_value = request.headers.iter().find_map(|header| {
        if header.name == "User-Agent" {
            Some(header.value.clone())
        } else {
            None
        }
    });
    let Some(user_agent_value) = user_agent_value else {
        anyhow::bail!("Failed to find User-Agent header in request")
    };
    let response = HttpResponse::new(StatusCode::Ok, Body::Text(user_agent_value));
    Ok(response)
}

fn handle_route_files(
    request: &HttpRequest,
    options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    // Look for the directory option
    let directory_path = options.iter().find_map(|option| {
        let CliOption::Directory(directory_path) = option;
        Some(directory_path)
    });
    let Some(directory_path) = directory_path else {
        return Ok(HttpResponse::not_found());
    };

    // Get the filename from the request path
    let path_parts: Vec<&str> = request.path.split(MAIN_SEPARATOR).collect();
    if path_parts.len() < 3 {
        return Ok(HttpResponse::not_found());
    }

    // Create the file path
    let mut file_path = directory_path.clone();
    if !file_path.ends_with(MAIN_SEPARATOR) {
        file_path.push(MAIN_SEPARATOR);
    }
    let filename = path_parts[2..].join(MAIN_SEPARATOR_STR);
    file_path.push_str(&filename);

    // Read the file
    let Ok(file_content) = fs::read(file_path) else {
        return Ok(HttpResponse::not_found());
    };

    Ok(HttpResponse::new(
        StatusCode::Ok,
        Body::BinaryData(file_content),
    ))
}
