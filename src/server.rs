use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::server::response::{HttpResponse, StatusCode};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

mod request;
mod response;

pub fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;

        // Read the request
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let request = request::HttpRequest::try_from(&buffer[..bytes_read])?;
        // println!("--> Received {:#?}", request);

        // Send the response
        let response = match request.path.as_str() {
            "/" => HttpResponse::new(StatusCode::Ok),
            _ => HttpResponse::new(StatusCode::NotFound),
        };
        // println!("--> Response {:#?}", response);
        let response: Vec<u8> = response.into();
        stream.write_all(&response)?;
        stream.flush()?;
    }

    Ok(())
}
