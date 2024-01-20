use std::fmt::Display;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
}

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => anyhow::bail!("Invalid method: {}", value),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method = match self {
            Method::GET => "GET",
            Method::POST => "POST",
        };
        write!(f, "{}", method)
    }
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl TryFrom<&str> for Header {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(": ").collect();
        if parts.len() != 2 {
            anyhow::bail!("Failed decoding header: {}", value);
        }
        Ok(Header {
            name: parts[0].to_string(),
            value: parts[1].to_string(),
        })
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub body: Option<Vec<u8>>,
}

impl TryFrom<&[u8]> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request_string = String::from_utf8_lossy(value).to_string();

        let mut lines = request_string.lines();
        let Some(request_line) = lines.next() else {
            anyhow::bail!("Failed decoding request");
        };
        let mut request_line_parts = request_line.split(" ");

        let Some(method) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request");
        };
        let method = Method::try_from(method)?;

        let Some(path) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request");
        };
        let path = path.to_string();

        let Some(http_version) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request");
        };
        let http_version = http_version.to_string();

        let headers: Vec<Header> = lines
            .clone()
            .take_while(|line| !line.is_empty())
            .filter_map(|line| Header::try_from(line).ok())
            .collect();

        let content_length_header = headers
            .iter()
            .find(|header| header.name == "Content-Length");

        let body = match content_length_header {
            Some(content_length_header) => {
                let content_length = content_length_header.value.parse::<usize>()?;
                let body = value[value.len() - content_length..].to_vec();
                Some(body)
            }
            None => None,
        };

        Ok(HttpRequest {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut request_string = format!("{} {} {}\r\n", self.method, self.path, self.http_version);
        for header in &self.headers {
            request_string += format!("{}: {}\r\n", header.name, header.value).as_str();
        }
        request_string += "\r\n";
        write!(f, "{}", request_string)
    }
}

impl HttpRequest {
    pub fn get_header(&self, name: &str) -> Option<&Header> {
        self.headers.iter().find(|header| header.name == name)
    }
}
