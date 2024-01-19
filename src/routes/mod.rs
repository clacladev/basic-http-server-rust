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
    let path_parts: Vec<&str> = request.path.split("/").collect();
    let body = path_parts[2..].join("/");
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
    _options: Arc<Vec<CliOption>>,
) -> anyhow::Result<HttpResponse> {
    let path_parts: Vec<&str> = request.path.split("/").collect();
    if path_parts.len() < 3 {
        return Ok(HttpResponse::not_found());
    }

    let filename = path_parts[2..].join("/");
    let response = HttpResponse::new(StatusCode::Ok, Body::Text(filename));
    Ok(response)
}
