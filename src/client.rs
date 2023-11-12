use log::{debug, warn};
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::result::Result;
use uuid::Uuid;

use crate::command::Command;
use crate::user::User;

pub struct Client {
    id: Uuid,
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    user: Option<User>,
    on_user_set: Option<Box<dyn FnMut(&str) -> Result<(), std::io::Error> + Send>>,
    on_user_unset: Option<Box<dyn FnMut(&str) -> Result<(), std::io::Error> + Send>>,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client {
            id: self.id.clone(),
            stream: self.stream.try_clone().expect("Failed to clone stream"),
            reader: BufReader::new(self.stream.try_clone().expect("Failed to clone stream")),
            user: self.user.clone(),
            on_user_set: None,
            on_user_unset: None,
        }
    }
}

impl Client {
    pub fn new(
        stream: TcpStream,
        on_user_set_callback: Option<Box<dyn FnMut(&str) -> Result<(), std::io::Error> + Send>>,
        on_user_unset_callback: Option<Box<dyn FnMut(&str) -> Result<(), std::io::Error> + Send>>,
    ) -> Client {
        let reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
        Client {
            id: Uuid::new_v4(),
            stream,
            reader,
            user: None,
            on_user_set: on_user_set_callback,
            on_user_unset: on_user_unset_callback,
        }
    }

    pub fn read_command(&mut self) -> Result<Option<Command>, std::io::Error> {
        let mut line = String::new();
        let len = self.reader.read_line(&mut line)?;

        if len == 0 {
            return Ok(None);
        }

        // Remove CRLF line ending which is standard for IRC protocol
        let line = line.trim_end_matches("\r\n");
        let command = Command::parse(line);
        Ok(Some(command))
    }

    pub fn send_response(&mut self, response: &str) -> Result<(), std::io::Error> {
        let mut message = response.to_string();

        // Ensure the message ends with CRLF
        if !message.ends_with("\r\n") {
            message.push_str("\r\n");
        }

        self.stream.write_all(message.as_bytes())?;

        // Ensure all data is sent immediately
        self.stream.flush() 
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user(&self) -> &Option<User> {
        &self.user
    }

    pub fn set_user(&mut self, user: User) {
        self.user = Some(user);

        if let Some(ref mut on_user_set) = self.on_user_set.take() {
            let nick_name = self.user.as_ref().unwrap().nick_name();
            let _ = on_user_set(nick_name);
        } else {
            warn!("on_user_set event handler is not set")
        }
    }
}

pub struct ClientStore {
    clients: HashMap<Uuid, Client>,
    nick_names: HashSet<String>,
}

impl ClientStore {
    pub fn new() -> ClientStore {
        ClientStore {
            clients: HashMap::new(),
            nick_names: HashSet::new(),
        }
    }

    pub fn add(&mut self, client: Client) {
        self.clients.insert(client.id().clone(), client);
    }

    pub fn remove(&mut self, client_id: &Uuid) -> Option<Client> {
        self.remove(client_id)
    }

    pub fn nick_name_exists(&self, nick_name: &str) -> bool {
        self.nick_names.contains(nick_name)
    }

    pub fn add_nick_name(&mut self, nick_name: &str) {
        debug!("add_nick_name: {}", nick_name);

        self.nick_names.insert(nick_name.to_string());

        // TODO: remove this debug code
        for nick_name in &self.nick_names {
            debug!("registered nick_name: {}", nick_name);
        }
    }

    pub fn remove_nick_name(&mut self, nick_name: &str) {
        self.nick_names.remove(nick_name);

        // TODO: remove this debug code
        for nick_name in &self.nick_names {
            debug!("registered nick_name: {}", nick_name);
        }
    }

    pub fn get_clients_from_users(&self, nick_names: &Vec<String>) -> Vec<&Client> {
        let mut clients = Vec::new();

        for nick_name in nick_names {
            // filter clients by user
            let client = self.clients.values().find(|client| {
                if let Some(client_user) = client.user() {
                    client_user.nick_name() == nick_name
                } else {
                    false
                }
            });

            if let Some(client) = client {
                clients.push(client);
            }
        }

        clients
    }
}
