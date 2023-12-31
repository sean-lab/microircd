pub struct User {
    nick_name: String,
    user_name: String,
    real_name: String,
}

impl User {
    pub fn new(nick_name: String, user_name: String, real_name: String) -> User {
        User {
            nick_name,
            user_name,
            real_name,
        }
    }

    pub fn nick_name(&self) -> &str {
        &self.nick_name
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }

    pub fn real_name(&self) -> &str {
        &self.real_name
    }

    pub fn set_real_name(&mut self, real_name: String) {
        self.real_name = real_name;
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            nick_name: self.nick_name.clone(),
            user_name: self.user_name.clone(),
            real_name: self.real_name.clone(),
        }
    }
}
