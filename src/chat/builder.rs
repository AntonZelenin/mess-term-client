use std::collections::HashMap;
use crate::chat::{Chat, Message};
use crate::helpers::types::{ChatId, UserId};
use crate::schemas::{ChatModel, MessageModel, User};

pub struct UserProvider {
    users: HashMap<UserId, User>,
}

impl UserProvider {
    pub fn new(users: Vec<User>) -> Self {
        let users = users.into_iter().map(|user| (user.id.clone(), user)).collect();
        UserProvider { users }
    }

    pub fn get_user(&self, user_id: &str) -> User {
        self.users.get(user_id).unwrap().clone()
    }
}

pub struct ChatBuilder {
    current_user: Option<User>,
    user_provider: UserProvider,
}

impl ChatBuilder {
    pub fn new(current_user: Option<User>, username_provider: UserProvider) -> Self {
        ChatBuilder {
            current_user,
            user_provider: username_provider,
        }
    }
    
    pub fn add_users(&mut self, user: Vec<User>) {
        for user in user {
            self.user_provider.users.insert(user.id.clone(), user);
        }
    }
    
    pub fn build_chats_from_models(&self, chat_models: Vec<ChatModel>) -> Vec<Chat> {
        chat_models.into_iter().map(|chat_model| self.build_chat_from_model(chat_model)).collect()
    }

    pub fn build_chat_from_model(&self, chat_model: ChatModel) -> Chat {
        let members = chat_model
            .member_ids
            .iter()
            .map(|user_id| self.user_provider.get_user(user_id))
            .collect();
       
        Chat {
            internal_id: chat_model.id.to_string(),
            id: Some(chat_model.id),
            name: self.get_chat_name(&chat_model),
            members,
            last_message: chat_model.messages.last().map(|message_model| self.build_message_from_model(message_model.clone())),
            number_of_unread_messages: 0,
        }
    }

    pub fn build_messages_from_models(&self, message_models: HashMap<ChatId, Vec<MessageModel>>) -> HashMap<ChatId, Vec<Message>> {
        message_models
            .into_iter()
            .map(|(chat_id, messages)|
                (chat_id, messages.into_iter().map(|message| self.build_message_from_model(message)).collect())
            )
            .collect()
    }

    pub fn build_message_from_model(&self, message_model: MessageModel) -> Message {
        Message {
            chat_id: message_model.chat_id,
            sender_username: self.user_provider.get_user(&message_model.sender_id).username,
            text: message_model.text.clone(),
            created_at: message_model.created_at,
            is_read: message_model.is_read,
        }
    }

    fn get_chat_name(&self, chat_model: &ChatModel) -> String {
        // If the chat has a name, return it. Otherwise, return the other member's username
        // Group chats will always have a name
        

        return if let Some(name) = chat_model.name.as_ref() {
            name.to_string()
        } else {
            assert_eq!(chat_model.member_ids.len(), 2, "Chat name is None and chat has more or less than 2 members");

            let another_user_id = chat_model
                .member_ids
                .iter()
                .find(|other_user_id| *other_user_id != &self.current_user.as_ref().expect("Cannot load usernames when unauthenticated").id)
                .unwrap();

            self.user_provider.get_user(another_user_id).username.clone()
        };
    }
}
