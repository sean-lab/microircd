use std::sync::Arc;

use log::info;

use crate::{
    channel::Channel, client::Client, error_response_code::IrcErrorCode,
    server_context::ServerContext,
};

pub fn handle_join(client: &mut Client, channel_name: String, server_context: Arc<ServerContext>) {
    let mut channel_store = server_context.channel_store.lock().unwrap();
    let user = client.user();
    let nick_name = user.clone().unwrap().nick_name().to_string();

    if channel_store.is_user_subscribed_to_channel(&nick_name, &channel_name) {
        let response = IrcErrorCode::ErrUserOnChannel.error_message(&channel_name);
        let _ = client.send_response(&response);
        return;
    }

    let subscribed_channels = channel_store.get_user_subscriptions(&nick_name);
    match subscribed_channels {
        Some(subscribed_channels) => {
            if subscribed_channels.len() >= 10 {
                let response = IrcErrorCode::ErrTooManyChannels.error_message(&channel_name);
                let _ = client.send_response(&response);
                return;
            }
        }
        None => {
            info!("{} is not subscribed to any channels", &nick_name);
        }
    }

    let channel: &mut Channel = channel_store.get_or_create_channel(&channel_name);
    channel.add_user(nick_name.clone());

    let response = format!(":{} JOIN :{}", nick_name, channel.name());
    let _ = client.send_response(&response);
}
