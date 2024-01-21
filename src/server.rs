use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{cli::CliOption, routes::handle_request};

use self::response::HttpResponse;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;
const MB: usize = 1024 * 1024;

pub mod request;
pub mod response;

pub async fn start_server(options: Vec<CliOption>) -> anyhow::Result<()> {
    let options = Arc::new(options);

    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).await?;
    println!("-> Server started on {}:{}", DEFAULT_IP, DEFAULT_PORT);

    loop {
        let (mut stream, _) = listener.accept().await?;
        let options = Arc::clone(&options);
        tokio::spawn(async move {
            let result: anyhow::Result<()> = handle_stream(&mut stream, options).await;
            match result {
                Ok(_) => Ok(()),
                Err(error) => handle_error(&mut stream, error).await,
            }
        });
    }
}

async fn handle_stream(stream: &mut TcpStream, options: Arc<Vec<CliOption>>) -> anyhow::Result<()> {
    // Read the request
    let mut buffer = Vec::with_capacity(1 * MB); // 1 MB
    let bytes_read = stream.read_buf(&mut buffer).await?;
    let Ok(request) = request::HttpRequest::try_from(&buffer[..bytes_read]) else {
        anyhow::bail!("Failed to parse request");
    };
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

async fn handle_error(stream: &mut TcpStream, error: anyhow::Error) -> anyhow::Result<()> {
    let response = HttpResponse::new(
        response::StatusCode::InternalServerError,
        response::Body::Text(error.to_string()),
    );
    let response_bytes = response.to_bytes();
    println!(
        "--> Sent response:\n{}",
        String::from_utf8_lossy(&response_bytes)
    );
    stream.write_all(&response_bytes).await?;
    Ok(())
}
