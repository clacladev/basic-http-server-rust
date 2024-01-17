use crate::server::request::HttpRequest;
use crate::server::response::{HttpResponse, StatusCode};

pub fn handle_request(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    match request.path.as_str() {
        "/" => handle_root(request),
        s if s == "/echo" || s.starts_with("/echo/") => handle_echo(request),
        _ => handle_unknown(request),
    }
}

fn handle_root(_request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::ok())
}

fn handle_echo(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    let path_parts: Vec<&str> = request.path.split("/").collect();
    let body = path_parts[2..].join("/");
    let response = HttpResponse::new(StatusCode::Ok, Some(body));
    Ok(response)
}

fn handle_unknown(_request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::not_found())
}
