use log::info;

use crate::{client::Client, command_response_code::IrcCommandResponse};

pub fn handle_ping(client: &mut Client, server: String) {
    // Respond to PING with PONG
    let response = format!("PONG {}", server);
    let _ = client.send_response(&response);
}

pub fn handle_pong(client: &mut Client, server: String) {
    // no response is required for PONG
    info!("Received PONG from {}", server);
}
