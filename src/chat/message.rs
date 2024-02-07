use serde::{Deserialize, Serialize};
use crate::helpers::types::ChatId;

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
    pub sent_at: f32,
    pub is_unread: bool,
}
