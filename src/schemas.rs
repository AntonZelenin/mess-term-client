use serde::{Deserialize, Serialize};
use crate::helpers::types::ChatId;

#[derive(Serialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RefreshTokenData {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatModel {
    pub id: ChatId,
    pub name: Option<String>,
    pub member_usernames: Vec<String>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewChatModel {
    pub name: Option<String>,
    pub member_usernames: Vec<String>,
    pub first_message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewMessage {
    pub chat_id: ChatId,
    pub sender_username: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub chat_id: u32,
    pub sender_username: String,
    pub text: String,
    pub created_at: f32,
    pub is_read: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSearchResults {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatSearchResults {
    pub chats: Vec<ChatModel>,
}
