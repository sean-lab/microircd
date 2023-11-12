use crate::client::ClientStore;

use crate::channel::ChannelStore;

use std::sync::{Arc, Mutex};

pub struct ServerContext {
    pub channel_store: Arc<Mutex<ChannelStore>>,
    pub client_store: Arc<Mutex<ClientStore>>,
}

impl ServerContext {
    pub fn new() -> ServerContext {
        ServerContext {
            channel_store: Arc::new(Mutex::new(ChannelStore::new())),
            client_store: Arc::new(Mutex::new(ClientStore::new())),
        }
    }
}
