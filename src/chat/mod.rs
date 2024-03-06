pub mod builder;
pub mod manager;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::schemas::User;
use crate::helpers::types::ChatId;
use crate::helpers::traits::InternalID;

// This chat represents both the ChatModel and the NewChatModel
#[derive(Clone)]
pub struct Chat {
    pub internal_id: String,
    pub id: Option<ChatId>,
    pub name: String,
    pub members: Vec<User>,
    pub last_message: Option<Message>,
    pub number_of_unread_messages: u32,
}

impl InternalID for Chat {
    fn internal_id(&self) -> String {
        self.internal_id.clone()
    }
}

impl Eq for Chat {}

impl PartialEq for Chat {
    fn eq(&self, other: &Self) -> bool {
        if self.last_message.is_none() || other.last_message.is_none() {
            return false;
        }
        self.last_message.as_ref().unwrap().created_at == other.last_message.as_ref().unwrap().created_at
    }
}

impl PartialOrd for Chat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.last_message.is_none() && other.last_message.is_none() {
            return Ordering::Equal;
        }
        if self.last_message.is_none() {
            return Ordering::Greater;
        }
        if other.last_message.is_none() {
            return Ordering::Less;
        }
        if self.last_message.as_ref().unwrap().created_at < other.last_message.as_ref().unwrap().created_at {
            return Ordering::Less;
        }
        if self.last_message.as_ref().unwrap().created_at > other.last_message.as_ref().unwrap().created_at {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub chat_id: u32,
    pub sender_username: String,
    pub text: String,
    pub created_at: f64,
    pub is_read: bool,
}
