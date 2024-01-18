#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Ok => "200 OK".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Body {
    Text(String),
    None,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    status_code: StatusCode,
    body: Body,
}

impl HttpResponse {
    pub fn ok() -> Self {
        HttpResponse {
            status_code: StatusCode::Ok,
            body: Body::None,
        }
    }

    pub fn not_found() -> Self {
        HttpResponse {
            status_code: StatusCode::NotFound,
            body: Body::None,
        }
    }

    pub fn new(status_code: StatusCode, body: Body) -> Self {
        HttpResponse { status_code, body }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let mut response_string = format!("HTTP/1.1 {}\r\n", self.status_code.to_string());

        match &self.body {
            Body::Text(body) => {
                response_string += "Content-Type: text/plain\r\n";
                response_string += format!("Content-Length: {}\r\n\r\n", body.len()).as_str();
                response_string += format!("{}\r\n", body).as_str();
            }
            Body::None => {}
        }

        response_string += "\r\n";
        response_string
    }
}
