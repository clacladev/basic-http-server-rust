use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{cli::CliOption, routes::handle_request};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

pub mod request;
pub mod response;

pub async fn start_server(options: Vec<CliOption>) -> anyhow::Result<()> {
    let options = Arc::new(options);

    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).await?;
    println!("-> Server started on {}:{}", DEFAULT_IP, DEFAULT_PORT);

    loop {
        let (stream, _) = listener.accept().await?;
        let options = Arc::clone(&options);
        tokio::spawn(async move { handle_stream(stream, options).await });
    }
}

async fn handle_stream(mut stream: TcpStream, options: Arc<Vec<CliOption>>) -> anyhow::Result<()> {
    // Read the request
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    let request = request::HttpRequest::try_from(&buffer[..bytes_read])?;
    println!("--> Received request:\n{}", request.to_string());

    // Send the response
    let response = handle_request(&request, options)?;
    let response_bytes = response.to_bytes();
    println!(
        "--> Sent response:\n{}",
        String::from_utf8_lossy(&response_bytes)
    );
    stream.write_all(&response_bytes).await?;

    Ok(())
}
