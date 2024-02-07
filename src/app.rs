use std::collections::HashMap;
use crossterm::event::KeyEvent;
use crate::{api, window};
use crate::chat::{Chat, NewChatModel};
use crate::chat::manager::ChatManager;
use crate::chat::message::{Message, NewMessage};
use crate::helpers::types::ChatId;
use crate::window::InputEntity;
use crate::window::login::{LoginTabs, LoginWindow};
use crate::window::main::MainWindow;

pub struct App {
    pub login_window: LoginWindow,
    pub main_window: MainWindow,
    active_window: Windows,
    loaded_internal_chat_id: Option<String>,
    api_client: api::Client,
    should_quit: bool,
    username: String,
}

impl App {
    pub async fn new(
        mut api_client: api::Client,
    ) -> Self {
        let mut chat_manager = ChatManager::new();

        if api_client.is_authenticated() {
            // contacts = Self::load_contacts(&mut api_client);
            let (chats, messages) = Self::load_chats_and_messages(&mut api_client).await;
            chat_manager.add_chats(chats);
            chat_manager.add_messages(messages);
        }

        Self {
            login_window: LoginWindow::default(),
            main_window: MainWindow::default(),
            active_window: if !api_client.is_authenticated() { Windows::Login } else { Windows::Main },
            loaded_internal_chat_id: None,
            api_client,
            should_quit: false,
            username: Self::load_username(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_client.is_authenticated()
    }

    pub fn tick(&mut self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    fn get_active_input_entity(&mut self) -> &mut dyn InputEntity {
        match self.active_window {
            Windows::Login => &mut self.login_window,
            Windows::Main => &mut self.main_window,
        }
    }

    pub fn pass_input_to_active_entity(&mut self, key_event: KeyEvent) {
        self.get_active_input_entity().process_input(key_event);
    }

    pub async fn submit(&mut self) {
        match self.active_window {
            Windows::Login => {
                match self.login_window.active_tab {
                    LoginTabs::Login => {
                        self.process_login().await;
                    }
                    LoginTabs::Register => {
                        self.process_register();
                    }
                }
            }
            Windows::Main => {
                match self.main_window.get_active_input_entity() {
                    window::main::ActiveInputEntity::SearchChats => {
                        if let Some(chat) = self.main_window.chat_manager.get_selected_chat() {
                            self.loaded_internal_chat_id = Some(chat.internal_id.clone());
                            self.main_window.set_active_input_entity(window::main::ActiveInputEntity::EnterMessage);
                        } else {
                            let search_results = self.api_client.search_chats_and_users(self.main_window.get_active_input()).await.unwrap();

                            let mut chats = search_results
                                .chats
                                .iter()
                                .map(|chat| Chat::from_model(chat.clone()))
                                .collect::<Vec<Chat>>();

                            for user in search_results.users {
                                chats.push(Chat {
                                    internal_id: user.username.clone(),
                                    id: None,
                                    name: user.username.clone(),
                                    member_usernames: vec![user.username.clone()],
                                    last_message: None,
                                    number_of_unread_messages: 0,
                                });
                            }

                            self.main_window.chat_manager.set_search_results(chats);
                        }
                    }
                    window::main::ActiveInputEntity::EnterMessage => {
                        let message_str = self.main_window.pop_message_input();
                        // todo new chats do not have id.. will it contain None for new chats?
                        let chat = self.main_window.chat_manager.get_selected_chat().unwrap();
                        if chat.id.is_some() {
                            let message = NewMessage {
                                chat_id: chat.id.unwrap(),
                                text: message_str,
                                sender_username: self.username.clone(),
                            };
                            self.send_message(message).await;
                        } else {
                            let new_chat = NewChatModel {
                                name: chat.name.clone(),
                                creator_username: self.username.clone(),
                                member_usernames: chat.member_usernames.clone(),
                                first_message: message_str,
                            };
                            self.create_chat(new_chat).await;
                        }
                    }
                }
            }
        }
    }

    pub async fn receive_message(&mut self) {
        if !self.api_client.is_authenticated() {
            return;
        }

        if let Some(message) = self.api_client.receive_message().await {
            self.main_window.chat_manager.add_message(message);
        }
    }

    async fn process_login(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        match self.api_client.login(&res["username"], &res["password"]).await {
            Ok(_) => {
                let (chats, messages) = Self::load_chats_and_messages(&mut self.api_client).await;
                self.main_window.chat_manager.add_chats(chats);
                self.main_window.chat_manager.add_messages(messages);

                self.active_window = Windows::Main;
            }
            Err(e) => {
                self.login_window.login_error_message = e;
            }
        }
    }

    async fn process_register(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        let error = self.validate_register_input(&res);
        if !error.is_empty() {
            self.login_window.register_error_message = error;
            return;
        }

        match self.api_client.register(&res["username"], &res["password"]).await {
            Ok(_) => {
                let (chats, messages) = Self::load_chats_and_messages(&mut self.api_client).await;
                self.main_window.chat_manager.add_chats(chats);
                self.main_window.chat_manager.add_messages(messages);

                self.active_window = Windows::Main;
            }
            Err(e) => {
                self.login_window.register_error_message = e;
            }
        }
    }

    fn validate_register_input(&self, input_values: &HashMap<String, String>) -> String {
        // todo returning only one error message is temporary
        let mut error_message = String::new();

        if input_values["password"] != input_values["password_confirmation"] {
            error_message.push_str("Passwords do not match.\n");
            return error_message;
        }

        if input_values["password"].len() < 8 {
            error_message.push_str("Password must be at least 8 characters long.\n");
            return error_message;
        }

        if input_values["username"].len() < 3 {
            error_message.push_str("Username must be at least 3 characters long.\n");
            return error_message;
        }

        error_message
    }

    async fn send_message(&mut self, message: NewMessage) {
        self.api_client.send_message(message).await;
    }

    async fn create_chat(&mut self, chat: NewChatModel) {
        let chat_model = self.api_client.create_chat(chat).await.unwrap();
        let chat = Chat::from_model(chat_model);
        self.main_window.chat_manager.add_chat(chat);
    }

    async fn load_chats_and_messages(api_client: &mut api::Client) -> (Vec<Chat>, HashMap<ChatId, Vec<Message>>) {
        let chat_models = api_client.get_chats().await.unwrap();

        let mut messages = HashMap::new();
        for chat in chat_models.iter() {
            messages.insert(chat.id, chat.messages.clone());
        }
        let mut chats = Vec::new();
        for chat_model in chat_models {
            let chat = Chat::from_model(chat_model);
            chats.push(chat);
        }

        (chats, messages)
    }

    fn load_username() -> String {
        // todo
        "username".to_string()
    }
}

pub enum Windows {
    Login,
    Main,
}
