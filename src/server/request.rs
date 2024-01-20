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
    raw_string: String,
}

impl TryFrom<&[u8]> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let request_string = String::from_utf8(value.to_vec())?;
        HttpRequest::try_from(request_string.as_str())
    }
}

impl TryFrom<&str> for HttpRequest {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let Some(request_line) = lines.next() else {
            anyhow::bail!("Failed decoding request line: {}", value);
        };
        let mut request_line_parts = request_line.split(" ");

        let Some(method) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request line: {}", value);
        };
        let method = Method::try_from(method)?;

        let Some(path) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request line: {}", value);
        };
        let path = path.to_string();

        let Some(http_version) = request_line_parts.next() else {
            anyhow::bail!("Failed decoding request line: {}", value);
        };
        let http_version = http_version.to_string();

        let headers = lines
            .filter_map(|line| Header::try_from(line).ok())
            .collect();

        Ok(HttpRequest {
            method,
            path,
            http_version,
            headers,
            raw_string: value.to_string(),
        })
    }
}

impl ToString for HttpRequest {
    fn to_string(&self) -> String {
        self.raw_string.clone()
    }
}
