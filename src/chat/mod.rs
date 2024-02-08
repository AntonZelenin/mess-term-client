pub mod manager;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::schemas::Message;
use crate::helpers::types::ChatId;
use crate::helpers::traits::InternalID;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatModel {
    pub id: ChatId,
    pub name: String,
    pub member_usernames: Vec<String>,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewChatModel {
    pub name: String,
    pub creator_username: String,
    pub member_usernames: Vec<String>,
    pub first_message: String,
}

// This chat represents both the ChatModel and the NewChatModel
#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub internal_id: String,
    pub id: Option<ChatId>,
    pub name: String,
    pub member_usernames: Vec<String>,
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
        self.last_message.as_ref().unwrap().sent_at == other.last_message.as_ref().unwrap().sent_at
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
        if self.last_message.as_ref().unwrap().sent_at < other.last_message.as_ref().unwrap().sent_at {
            return Ordering::Less;
        }
        if self.last_message.as_ref().unwrap().sent_at > other.last_message.as_ref().unwrap().sent_at {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Chat {
    pub fn from_model(chat_model: ChatModel) -> Self {
        let last_message = chat_model.messages.last().cloned();
        let number_of_unread_messages = chat_model.messages.iter().filter(|m| m.is_unread).count() as u32;

        Chat {
            internal_id: chat_model.id.to_string(),
            id: Some(chat_model.id),
            name: chat_model.name,
            member_usernames: chat_model.member_usernames,
            last_message,
            number_of_unread_messages,
        }
    }
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
