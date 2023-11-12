use std::collections::{HashMap, HashSet};

use log::{debug, info};

type ChannelEventHandler = fn(&mut ChannelStore, String, String);

pub struct Channel {
    name: String,
    users: Vec<String>,
    on_join: Option<ChannelEventHandler>,
    on_leave: Option<ChannelEventHandler>,
}

impl Channel {
    fn new(
        name: String,
        on_join: Option<ChannelEventHandler>,
        on_leave: Option<ChannelEventHandler>,
    ) -> Self {
        Self {
            name,
            users: Vec::new(),
            on_join: on_join,
            on_leave: on_leave,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_user(&mut self, user: String) {
        self.users.push(user);
    }

    pub fn contains_user(&self, user: &str) -> bool {
        self.users.contains(&user.to_string())
    }

    pub fn remove_user(&mut self, user: &str) {
        self.users.retain(|u| u != user);
    }

    pub fn get_users(&self) -> Vec<String> {
        self.users.clone()
    }
}

pub struct ChannelStore {
    channels: HashMap<String, Channel>,
    user_subscriptions: HashMap<String, HashSet<String>>,
}

impl ChannelStore {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
            user_subscriptions: HashMap::new(),
        }
    }

    fn on_join(&mut self, nick_name: String, channel_name: String) {
        if !self.user_subscriptions.contains_key(&nick_name) {
            self.user_subscriptions
                .insert(nick_name.clone(), HashSet::new());
            debug!("user {} is not in user_subscriptions. Inserting", nick_name);
        }

        let user_subscription = self.user_subscriptions.get_mut(&nick_name).unwrap();
        user_subscription.insert(channel_name.clone());
        debug!(
            "user {} is now subscribed to channel {}",
            nick_name, channel_name
        );
    }

    fn on_leave(&mut self, nick_name: String, channel_name: String) {
        if self.user_subscriptions.contains_key(&nick_name) {
            let user_subscription = self.user_subscriptions.get_mut(&nick_name).unwrap();
            user_subscription.remove(&channel_name);
            debug!(
                "user {} is now unsubscribed from channel {}",
                nick_name, channel_name
            );
        }
    }

    pub fn get_user_subscriptions(&self, user: &str) -> Option<&HashSet<String>> {
        self.user_subscriptions.get(user)
    }

    pub fn is_user_subscribed_to_channel(&self, nick_name: &str, channel_name: &str) -> bool {
        if self.user_subscriptions.contains_key(nick_name) {
            self.user_subscriptions
                .get(nick_name)
                .unwrap()
                .contains(channel_name)
        } else {
            false
        }
    }

    pub fn get_or_create_channel(&mut self, channel_name: &str) -> &mut Channel {
        if !self.channels.contains_key(channel_name) {
            let channel = Channel {
                name: channel_name.to_string(),
                users: Vec::new(),
                on_join: Some(Self::on_join),
                on_leave: Some(Self::on_leave),
            };

            self.channels.insert(channel.name.clone(), channel);
            info!("channel {} created", channel_name);
        }

        self.channels.get_mut(channel_name).unwrap()
    }

    pub fn remove_channel(&mut self, channel: &str) {
        self.channels.remove(channel);
    }

    pub fn get_channel(&self, channel: &str) -> Option<&Channel> {
        self.channels.get(channel)
    }
}
