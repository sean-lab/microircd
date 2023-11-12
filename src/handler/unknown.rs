use log::warn;

use crate::client::Client;

pub fn handle_unknown(client: &mut Client, command: String) {
    warn!("Received unknown command: {}", command);
}
