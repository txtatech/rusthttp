// main.rs
mod server;
use server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new();
    server.run().await
}
