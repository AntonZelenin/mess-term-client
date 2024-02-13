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
