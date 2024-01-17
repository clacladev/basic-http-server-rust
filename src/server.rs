use std::{
    io::{Read, Write},
    net::TcpListener,
};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

pub fn start_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut message: Vec<u8> = Vec::new();
        let message_length = stream.read(&mut message)?;
        let message_string = String::from_utf8_lossy(&message).to_string();
        println!(
            "--> Received message ({}b long): {}",
            message_length, message_string
        );

        let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
        stream.write_all(response)?;
        println!("--> Response sent: 200 OK")
    }

    Ok(())
}
