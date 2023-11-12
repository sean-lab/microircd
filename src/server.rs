use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use crate::client::Client;
use crate::handler::handle_command;
use crate::server_context::ServerContext;

use log::{debug, error, info};

pub struct Server {
    address: String,
    context: Arc<ServerContext>,
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_string(),
            context: Arc::new(ServerContext::new()),
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address)?;

        info!("Server running on {}", &self.address);

        // Accept connections and process them
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let channel_store = Arc::clone(&self.context);

                    thread::spawn(move || {
                        if let Err(e) = Self::handle_client(stream, channel_store) {
                            error!("connection failed: {}", e);
                        }
                    });
                }
                Err(e) => error!("connection failed: {}", e),
            }
        }

        Ok(())
    }

    fn handle_client(stream: TcpStream, server_context: Arc<ServerContext>) -> Result<()> {
        let server_context_for_user_set = Arc::clone(&server_context);
        let server_context_for_user_unset = Arc::clone(&server_context);
        let mut client = Client::new(
            stream,
            Some(Box::new(move |nick_name| {
                let client_store = Arc::clone(&server_context_for_user_set.client_store);
                client_store.lock().unwrap().add_nick_name(nick_name);
                Ok(())
            })),
            Some(Box::new(move |nick_name| {
                server_context_for_user_unset
                    .client_store
                    .lock()
                    .unwrap()
                    .remove_nick_name(nick_name);
                Ok(())
            })),
        );

        // Read commands from the client
        while let Some(command) = client.read_command()? {
            // Process the command by calling the appropriate handler
            debug!("handling command: {:?}", command);
            handle_command(&mut client, command, server_context.clone());
        }

        Ok(())
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
