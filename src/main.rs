use std::env;

use cli::CliOption;
use server::start_server;

mod cli;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let Ok(options) = CliOption::from_str(&args[1..]) else {
        panic!("Failed to parse options");
    };

    match start_server(options).await {
        Ok(_) => println!("-> Server stopped"),
        Err(e) => println!("-> Error: {}", e),
    }
}
