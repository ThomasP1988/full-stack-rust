use chrono::Utc;
use lazy_static::lazy_static; // 1.4.0
use shared::message::Message;
use std::sync::Mutex;

lazy_static! {
    static ref MESSAGES: Mutex<Vec<Message>> = Mutex::new(vec![]);
}

pub fn add(usernme: &str, message: &str) -> Message {
    let msg: Message = Message {
        username: usernme.to_owned(),
        message: message.to_owned(),
        date: Utc::now(),
    };

    MESSAGES.lock().unwrap().push(msg.clone());

    msg
}

 pub fn list(limit: usize) -> Vec<Message> {
    MESSAGES.lock().unwrap().iter().take(limit).cloned().collect()
}
