mod channel;
mod client;
mod command;
mod command_response_code;
mod error_response_code;
mod handler;
mod server;
mod server_context;
mod user;

use log::{error, info};
use server::Server;

fn main() {
    // If not set, the default log level is "warn".
    env_logger::init();

    // TODO: parameterize the address
    let server = Server::new("127.0.0.1:6667");
    info!("starting server on {}", server.address());
    if let Err(e) = server.run() {
        error!("Failed to start server: {}", e);
    }

    info!("shutting down");
}
