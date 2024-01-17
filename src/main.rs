use std::net::TcpListener;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 4221;

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
