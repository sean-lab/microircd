use std::sync::Arc;

use crate::{client::Client, error_response_code::IrcErrorCode, server_context};

pub fn handle_privmsg(
    client: &mut Client,
    target: String,
    message: String,
    server_context: Arc<server_context::ServerContext>,
) {
    let sender = client.user().as_ref().unwrap().nick_name();

    // channel names start with #
    if target.starts_with("#") {
        let channel_store = server_context.channel_store.lock().unwrap();
        let channel = channel_store.channel(&target);

        if channel.is_none() {
            let response = IrcErrorCode::ErrNoSuchChannel.error_message(&target);
            let _ = client.send_response(&response);
            return;
        }

        // get all users in channel except sender
        let nick_names = channel
            .unwrap()
            .users()
            .iter()
            .filter(|&x| x != &sender)
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        // get all clients in channel except sender by nick names
        let client_store = server_context.client_store.lock().unwrap();
        let peers_in_channel = client_store.get_clients_from_users(&nick_names);

        // send message to all clients
        let response = format!(":{} PRIVMSG {} {}", &sender, target, message);
        for peer in peers_in_channel {
            let _ = peer.clone().send_response(&response);
        }
    } else if !target.starts_with("$") {
        // server name starts with $, by negating the condition, we can check if the target is a user
        let response = format!("Message from {}: {}", &sender, message);
        let _ = client.send_response(&response);
    } else {
        // server names start with $. Currently, we don't support sending messages to other servers
        let response = IrcErrorCode::ErrNoSuchServer.error_message(&target);
        let _ = client.send_response(&response);
    }
}
