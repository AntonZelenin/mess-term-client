use std::collections::HashMap;
use crate::chat::{Chat, Message};
use crate::helpers::list::StatefulOrderedList;
use crate::helpers::types::ChatId;

#[derive(Default)]
pub struct ChatManager {
    chats: StatefulOrderedList<Chat>,
    messages: HashMap<ChatId, Vec<Message>>,
    search_results: StatefulOrderedList<Chat>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_chats(&mut self, chat: Vec<Chat>) {
        self.chats.extend(chat);
    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.chats.push(chat);
    }

    pub fn add_messages(&mut self, messages: HashMap<ChatId, Vec<Message>>) {
        for (chat_id, messages) in messages {
            let chat = self.chats.get_mut(&chat_id.to_string());
            // todo what if I read them right away? I mean if the chat is open
            chat.number_of_unread_messages += messages.len() as u32;

            self.messages.get_mut(&chat_id).expect("Chat messages not found").extend(messages);
        }
    }

    pub fn add_message(&mut self, message: Message) {
        let chat = self.chats.get_mut(&message.chat_id.to_string());

        chat.number_of_unread_messages += 1;

        self.messages.get_mut(&message.chat_id).expect("Chat messages not found").push(message);
    }

    pub fn select_chat(&mut self, chat_id: ChatId) -> Chat {
        self.chats.select(&chat_id.to_string())
    }

    pub fn unselect_chat(&mut self) {
        self.chats.unselect();
    }

    pub fn get_selected_chat(&self) -> Option<Chat> {
        if self.chats.selected_item_id.is_none() {
            return None;
        }

        let chat_id = self.chats.selected_item_id.as_ref().expect("Chat not found");
        Some(self.chats.get(chat_id).clone())
    }

    pub fn get_active_chats_mut(&mut self) -> &mut StatefulOrderedList<Chat> {
        if self.search_results.is_empty() {
            &mut self.chats
        } else {
            &mut self.search_results
        }
    }

    pub fn get_messages(&self, chat_id: ChatId) -> &Vec<Message> {
        self.messages.get(&chat_id).expect("Chat messages not found")
    }

    /// Search results combines both existing chats and users with which the user can start a new chat
    /// potential new chats do not have an id, only a random internal id
    pub fn set_search_results(&mut self, search_results: Vec<Chat>) {
        self.clear_search_results();
        self.search_results.extend(search_results);
    }

    pub fn clear_search_results(&mut self) {
        self.search_results = StatefulOrderedList::default();
    }

    pub fn read_all(&mut self, chat_id: ChatId) {
        let chat = self.chats.get_mut(&chat_id.to_string());
        chat.number_of_unread_messages = 0;
    }

    pub fn select_previous_chat(&mut self) {
        self.chats.previous();
    }

    pub fn select_next_chat(&mut self) {
        self.get_active_chats_mut().next();
    }
}
