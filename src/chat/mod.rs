pub mod manager;

use std::cmp::Ordering;
use crate::app;
use crate::schemas::{ChatModel, Message};
use crate::helpers::types::ChatId;
use crate::helpers::traits::InternalID;

// This chat represents both the ChatModel and the NewChatModel
#[derive(Clone)]
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

impl Chat {
    pub fn from_model(chat_model: ChatModel) -> Self {
        let last_message = chat_model.messages.first().cloned();
        let chat_name = if let Some(name) = chat_model.name {
            name
        } else {
            assert_eq!(chat_model.member_usernames.len(), 2, "Chat name is None and chat has more or less than 2 members");

            let current_username = app::get_username();
            chat_model.member_usernames.iter().find(|username| *username != &current_username).unwrap().clone()
        };

        Chat {
            internal_id: chat_model.id.to_string(),
            id: Some(chat_model.id),
            name: chat_name,
            member_usernames: chat_model.member_usernames,
            last_message,
            number_of_unread_messages: 0,
        }
    }
}
