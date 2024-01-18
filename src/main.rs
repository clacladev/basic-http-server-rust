use server::start_server;

mod routes;
mod server;

#[tokio::main]
async fn main() {
    match start_server().await {
        Ok(_) => println!("-> Server stopped"),
        Err(e) => println!("-> Error: {}", e),
    }
}
