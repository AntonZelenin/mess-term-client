use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub chat_id: u32,
    pub sender_id: String,
    pub text: String,
    pub created_at: f32,
}

impl Message {
    pub fn as_string(&self) -> String {
        self.text.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: u32,
    pub name: Option<String>,
    pub member_ids: Vec<String>,
    pub messages: Vec<Message>,
}
