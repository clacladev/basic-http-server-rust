use std::{fmt::Display, vec};

#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NotFound = 404,
    InternalServerError = 500,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_code = match self {
            StatusCode::Ok => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::InternalServerError => "500 Internal Server Error",
        };
        write!(f, "{}", status_code)
    }
}

#[derive(Debug, Clone)]
pub enum Body {
    Text(String),
    BinaryData(Vec<u8>),
    None,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    status_code: StatusCode,
    body: Body,
}

impl HttpResponse {
    pub fn ok() -> Self {
        HttpResponse::new(StatusCode::Ok, Body::None)
    }

    pub fn not_found() -> Self {
        HttpResponse::new(StatusCode::NotFound, Body::None)
    }

    pub fn new(status_code: StatusCode, body: Body) -> Self {
        HttpResponse { status_code, body }
    }
}

enum MimeType {
    TextPlain,
    ApplicationOctetStream,
}

impl Display for MimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mime_type = match self {
            MimeType::TextPlain => "text/plain",
            MimeType::ApplicationOctetStream => "application/octet-stream",
        };
        write!(f, "{}", mime_type)
    }
}

impl HttpResponse {
    pub fn to_bytes(&self) -> Vec<u8> {
        match &self.body {
            Body::Text(body) => {
                let mut response_string = format!("HTTP/1.1 {}\r\n", self.status_code);
                response_string += format!("Content-Type: {}\r\n", MimeType::TextPlain).as_str();
                response_string += format!("Content-Length: {}\r\n\r\n", body.len()).as_str();
                response_string += body;
                response_string += "\r\n\r\n";
                response_string.as_bytes().to_vec()
            }
            Body::BinaryData(data) => {
                let mut response_string = format!("HTTP/1.1 {}\r\n", self.status_code);
                response_string +=
                    format!("Content-Type: {}\r\n", MimeType::ApplicationOctetStream).as_str();
                response_string += format!("Content-Length: {}\r\n\r\n", data.len()).as_str();

                let mut parts_bytes: Vec<&[u8]> = vec![];
                parts_bytes.push(response_string.as_bytes());
                parts_bytes.push(data);
                parts_bytes.push("\r\n\r\n".as_bytes());
                parts_bytes.concat()
            }
            Body::None => {
                let response_string = format!("HTTP/1.1 {}\r\n\r\n", self.status_code);
                response_string.as_bytes().to_vec()
            }
        }
    }
}
