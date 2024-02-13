use std::collections::HashMap;
use crate::chat::{Chat, Message};
use crate::helpers::list::StatefulOrderedList;
use crate::helpers::types::ChatId;

#[derive(Default)]
pub struct ChatManager {
    chats: StatefulOrderedList<Chat>,
    messages: HashMap<ChatId, Vec<Message>>,
    search_results: StatefulOrderedList<Chat>,
    loaded_internal_chat_id: Option<String>,
}

impl ChatManager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_chats(&mut self, chats: Vec<Chat>) {
        for chat in chats.iter() {
            self.messages.insert(chat.id.clone().expect("Chat id not found"), vec![]);
        }
        self.chats.extend(chats);
    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.messages.insert(chat.id.clone().expect("Chat id not found"), vec![]);
        self.chats.push(chat);
    }

    pub fn add_messages(&mut self, messages: HashMap<ChatId, Vec<Message>>) {
        for (chat_id, messages) in messages {
            let chat = self.chats.get_mut(&chat_id.to_string());
            // todo what if I read them right away? I mean if the chat is open
            chat.number_of_unread_messages += messages.iter().filter(|m| !m.is_read).count() as u32;

            self.messages.get_mut(&chat_id).expect("Chat messages not found").extend(messages);
        }
    }

    pub fn add_message(&mut self, message: Message) {
        let chat = self.chats.get_mut(&message.chat_id.to_string());

        chat.number_of_unread_messages += if message.is_read { 0 } else { 1 };

        self.messages.get_mut(&message.chat_id).expect("Chat messages not found").push(message);
    }

    pub fn load_chat(&mut self, chat_id: String) {
        self.loaded_internal_chat_id = Some(chat_id.clone());
        let chats = self.get_active_chats_mut();
        if chats.contains(&chat_id) {
            let chat = chats.get_mut(&chat_id);
            chat.number_of_unread_messages = 0;
        }
    }

    pub fn select_chat(&mut self, chat_id: String) {
        let chats = self.get_active_chats_mut();
        chats.select(&chat_id);
    }

    pub fn unselect_chat(&mut self) {
        let chats = self.get_active_chats_mut();
        chats.unselect();
    }

    pub fn get_selected_chat(&self) -> Option<Chat> {
        let chats = self.get_active_chats();
        if chats.selected_item_id.is_none() {
            return None;
        }

        let chat_id = chats.selected_item_id.as_ref().expect("Chat not found");
        Some(chats.get(chat_id).clone())
    }

    pub fn get_loaded_chat(&self) -> Option<&Chat> {
        let chats = self.get_active_chats();
        if let Some(chat_id) = &self.loaded_internal_chat_id {
            return Some(chats.get(&chat_id));
        }
        None
    }

    pub fn unload_chat(&mut self) {
        self.loaded_internal_chat_id = None;
    }

    pub fn get_active_chats(&self) -> &StatefulOrderedList<Chat> {
        if self.search_results.is_empty() {
            &self.chats
        } else {
            &self.search_results
        }
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
        self.get_active_chats_mut().previous();
    }

    pub fn select_next_chat(&mut self) {
        self.get_active_chats_mut().next();
    }
}
