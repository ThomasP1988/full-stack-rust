use lazy_static::lazy_static; // 1.4.0
use shared::user::User;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    static ref USERS: Mutex<HashMap<String, User>> = Mutex::new(HashMap::new());
}

pub fn add(username: &str) -> User {
    let user: User = User {
        username: username.to_owned(),

    };

    USERS.lock().unwrap().insert(username.to_owned(), user.to_owned());

    user
}

 pub fn get(username: &str) -> Option<User> {
    USERS.lock().unwrap().get(username).cloned()
}
