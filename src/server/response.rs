#[derive(Debug, Clone)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
}

impl Into<&str> for StatusCode {
    fn into(self) -> &'static str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::NotFound => "404 Not Found",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    status_code: StatusCode,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode) -> Self {
        HttpResponse { status_code }
    }
}

impl ToString for HttpResponse {
    fn to_string(&self) -> String {
        let status_message: &str = self.status_code.clone().into();
        format!("HTTP/1.1 {}\r\n", status_message)
    }
}

impl Into<Vec<u8>> for HttpResponse {
    fn into(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}
