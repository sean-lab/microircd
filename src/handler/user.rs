use std::sync::Arc;

use log::info;

use crate::{client::Client, error_response_code::IrcErrorCode};

pub fn handle_user(client: &mut Client, user_name: String, real_name: String) {
    let mut user = client.user().clone();

    if user.is_none() {
        // According to RFC 1459 section 4.1.3, USER command must be sent after NICK command.
        let response = IrcErrorCode::ErrNoNicknameGiven.error_message(&user_name);
        info!("{} tried to set user name without a nick name", user_name);
        let _ = client.send_response(&response);
        return;
    }

    let user = user.as_mut().unwrap();
    user.set_user_name(user_name.clone());
    user.set_real_name(real_name.clone());

    let nick_name = user.nick_name();

    // According to RFC 1459 section 4.1.3, a server doesn't have a response for USER command.
    let response = format!(
        "now {}'s user name is {}, and real name is {}",
        nick_name, user_name, real_name
    );
    info!("{}", response);
    let _ = client.send_response(&response);
}
