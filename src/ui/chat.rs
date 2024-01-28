use crate::chat::{Chat, Message};
use crate::helpers::list::StatefulList;

pub struct StatefulChat {
    pub id: u32,
    pub name: String,
    pub messages: StatefulList<Message>,
}

impl StatefulChat {
    pub fn from_chat(chat: Chat) -> Self {
        Self {
            id: chat.id,
            name: chat.name,
            messages: StatefulList::with_items(chat.messages),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
