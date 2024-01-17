use std::{
    io::{Read, Write},
    net::TcpListener,
};

use crate::routes::handle_request;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

pub mod request;
pub mod response;

pub fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT))?;
    println!("-> Server started on {}:{}", DEFAULT_IP, DEFAULT_PORT);

    for stream in listener.incoming() {
        let mut stream = stream?;

        // Read the request
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let request = request::HttpRequest::try_from(&buffer[..bytes_read])?;
        println!("--> Received request:\n{}", request.to_string());

        // Send the response
        let response = handle_request(&request)?;
        println!("--> Sent response:\n{}", response.to_string());
        stream.write_all(response.to_string().as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
