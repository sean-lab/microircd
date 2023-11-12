use std::{error, sync::Arc};

use crate::command_response_code::IrcCommandResponse;
use crate::error_response_code::IrcErrorCode;
use log::info;

use crate::{client::Client, server_context::ServerContext, user::User};

pub fn handle_nick(client: &mut Client, nick_name: String, server_context: Arc<ServerContext>) {
    let client_store = server_context.client_store.lock().unwrap();
    if client_store.nick_name_exists(&nick_name) {
        let response = IrcErrorCode::ErrNicknameInUse.error_message(&nick_name);
        info!(
            "{} tried to join the server with an existing nick name",
            nick_name
        );
        let _ = client.send_response(&response);
        return;
    }

    drop(client_store);

    client.set_user(User::new(nick_name.clone(), String::new(), String::new()));

    let mut client_store = server_context.client_store.lock().unwrap();
    client_store.add(client.clone());

    let response = IrcCommandResponse::RplWelcome.format_response(&["Sean's", &nick_name]);
    info!("{} joined the server", nick_name);

    let _ = client.send_response(&response);
}
