use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::routes::handle_request;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

pub mod request;
pub mod response;

pub async fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).await?;
    println!("-> Server started on {}:{}", DEFAULT_IP, DEFAULT_PORT);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move { handle_stream(stream).await });
    }
}

async fn handle_stream(mut stream: TcpStream) -> anyhow::Result<()> {
    // Read the request
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    let request = request::HttpRequest::try_from(&buffer[..bytes_read])?;
    println!("--> Received request:\n{}", request.to_string());

    // Send the response
    let response = handle_request(&request)?;
    println!("--> Sent response:\n{}", response.to_string());
    stream.write_all(response.to_string().as_bytes()).await?;

    Ok(())
}
