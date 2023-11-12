// src/handler/mod.rs
mod join;
mod nick;
mod ping;
mod privmsg;
mod unknown;
mod user;

use std::sync::Arc;

use crate::client::Client;
use crate::command::Command;
use crate::server_context::ServerContext;

use join::handle_join;
use nick::handle_nick;
use ping::{handle_ping, handle_pong};
use privmsg::handle_privmsg;
use unknown::handle_unknown;
use user::handle_user;
// Add more imports as needed

pub fn handle_command(client: &mut Client, command: Command, server_context: Arc<ServerContext>) {
    match command {
        Command::Nick(nick_name) => handle_nick(client, nick_name, server_context),
        Command::User(user_name, _, _, real_name) => handle_user(client, user_name, real_name),
        Command::PrivMsg(target, message) => {
            handle_privmsg(client, target, message, server_context)
        }
        Command::Ping(server) => handle_ping(client, server),
        Command::Pong(server) => handle_pong(client, server),
        Command::Join(channel) => handle_join(client, channel, server_context),

        //---------------------------------//
        // TODO: Add more commands here
        //---------------------------------//
        Command::Unknown(unknown_command) => handle_unknown(client, unknown_command),
    }
}
