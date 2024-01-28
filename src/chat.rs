use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: u32,
    pub content: String,
}

impl Message {
    pub fn as_string(&self) -> String {
        self.content.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub id: u32,
    pub name: String,
    pub messages: Vec<Message>,
}
