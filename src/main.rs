use server::start_server;

mod server;

fn main() {
    match start_server() {
        Ok(_) => println!("-> Server stopped"),
        Err(e) => println!("-> Error: {}", e),
    }
}
