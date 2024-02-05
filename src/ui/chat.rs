use crate::chat::{Chat, Message};
use crate::helpers::list::StatefulList;

pub struct StatefulChat {
    pub id: Option<u32>,
    pub name: String,
    pub member_usernames: Vec<String>,
    pub messages: StatefulList<Message>,
}

impl StatefulChat {
    pub fn from_chat(chat: Chat) -> Self {
        Self {
            id: chat.id,
            name: chat.name.unwrap_or("change me".to_string()),
            member_usernames: chat.member_ids,
            messages: StatefulList::with_items(chat.messages),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        assert!(self.id.is_some());

        self.messages.push(message);
    }
}
