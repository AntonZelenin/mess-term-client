use crate::chat::builder::{ChatBuilder, UserProvider};
use crate::schemas::User;

pub fn get_chat_builder(users: Vec<User>, current_user: Option<User>) -> ChatBuilder {
    let user_provider = UserProvider::new(users);
    ChatBuilder::new(current_user, user_provider)
}
