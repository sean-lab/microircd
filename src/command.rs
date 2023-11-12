use log::debug;

#[derive(Debug)]
pub enum Command {
    Nick(String),
    User(String, String, String, String),
    PrivMsg(String, String),
    Ping(String),
    Pong(String),
    Join(String),
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Command {
        let mut parts = input.split_whitespace();

        match parts.next() {
            Some("NICK") => {
                let nickname = parts.next().unwrap_or("").to_string();
                Command::Nick(nickname)
            }
            Some("USER") => {
                let user_name = parts.next().unwrap_or("").to_string();
                let mode = parts.next().unwrap_or("").to_string();
                let unused = parts.next().unwrap_or("").to_string();
                let real_name = parts.collect::<Vec<&str>>().join(" ");
                Command::User(user_name, mode, unused, real_name)
            }
            Some("PRIVMSG") => {
                // Currently, target can only be a channel or a user
                // TODO: support sending messages to other servers
                let target = parts.next().unwrap_or("").to_string();
                let message = parts.collect::<Vec<&str>>().join(" ");
                Command::PrivMsg(target, message)
            }
            Some("PING") => {
                let server = parts.next().unwrap_or("").to_string();
                Command::Ping(server)
            }
            Some("PONG") => {
                let server = parts.next().unwrap_or("").to_string();
                Command::Pong(server)
            }
            Some("JOIN") => {
                let channel = parts.next().unwrap_or("").to_string();
                Command::Join(channel)
            }
            _ => Command::Unknown(input.to_string()),
        }
    }
}
