use crate::server::request::HttpRequest;
use crate::server::response::{Body, HttpResponse, StatusCode};

pub fn handle_request(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    match request.path.as_str() {
        "/" => handle_route_root(request),
        s if s == "/echo" || s.starts_with("/echo/") => handle_route_echo(request),
        s if s == "/user-agent" || s.starts_with("/user-agent/") => {
            handle_route_user_agent(request)
        }
        _ => handle_route_unknown(request),
    }
}

fn handle_route_root(_request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::ok())
}

fn handle_route_echo(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    let path_parts: Vec<&str> = request.path.split("/").collect();
    let body = path_parts[2..].join("/");
    let response = HttpResponse::new(StatusCode::Ok, Body::Text(body));
    Ok(response)
}

fn handle_route_user_agent(request: &HttpRequest) -> anyhow::Result<HttpResponse> {
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

fn handle_route_unknown(_request: &HttpRequest) -> anyhow::Result<HttpResponse> {
    Ok(HttpResponse::not_found())
}
