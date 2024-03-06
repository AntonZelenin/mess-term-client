use std::collections::{HashMap, HashSet};
use crossterm::event::KeyEvent;
use crate::{api, factory, helpers, storage, window};
use crate::api::ApiError;
use crate::chat::builder::ChatBuilder;
use crate::schemas::{ChatModel, Message, NewChatModel, NewMessage, User};
use crate::chat::Chat;
use crate::chat::manager::ChatManager;
use crate::helpers::types::{ChatId, TextInput};
use crate::window::InputEntity;
use crate::window::login::{LoginTabs, LoginWindow};
use crate::window::main::MainWindow;

pub struct App {
    pub login_window: LoginWindow,
    pub main_window: MainWindow,

    active_window: Windows,
    api_client: api::Client,
    chat_builder: ChatBuilder,
    should_quit: bool,
    user: Option<User>,
}

impl App {
    pub async fn new(
        mut api_client: api::Client,
    ) -> Self {
        let mut chat_manager = ChatManager::new();
        let mut chat_builder = factory::get_chat_builder(vec![], get_current_user());
        let mut user = None;

        if api_client.is_authenticated() {
            let (chat_models, messages) = Self::load_chats_and_messages(&mut api_client).await;
            let user_ids = extract_user_ids(chat_manager.get_chats());
            match api_client.get_users(user_ids).await {
                Ok(users_result) => {
                    chat_builder = factory::get_chat_builder(users_result.users, get_current_user());
                    chat_manager.add_chats(chat_builder.build_from_models(chat_models));
                    chat_manager.add_messages(messages);
                }
                Err(e) => panic!("Error while loading chat: {:?}", e),
            }

            user = get_current_user();
        }

        Self {
            login_window: LoginWindow::default(),
            main_window: MainWindow::new(chat_manager),
            active_window: if !api_client.is_authenticated() { Windows::Login } else { Windows::Main },
            api_client,
            chat_builder,
            should_quit: false,
            user,
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
                match self.login_window.selected_tab {
                    LoginTabs::Login => {
                        self.process_login().await;
                    }
                    LoginTabs::Register => {
                        self.process_register().await;
                    }
                }
            }
            Windows::Main => {
                match self.main_window.get_active_input_entity() {
                    window::main::ActiveInputEntity::SearchChats => {
                        self.run_search().await;
                    }
                    window::main::ActiveInputEntity::SelectChat => {
                        self.open_chat(
                            self.main_window.chat_manager.get_selected_chat().expect("Cannot open chat without a selected chat")
                        ).await;
                    }
                    window::main::ActiveInputEntity::EnterMessage => {
                        let message_str = helpers::input_to_string(&self.main_window.pop_message_input());
                        if message_str.is_empty() {
                            return;
                        }
                        // todo new chats do not have id.. will it contain None for new chats?
                        let chat = self.main_window.chat_manager.get_selected_chat().unwrap();
                        if chat.id.is_some() {
                            let message = NewMessage {
                                chat_id: chat.id.unwrap(),
                                text: message_str,
                                sender_id: self.user.as_ref().unwrap().id.clone(),
                            };
                            self.send_message(message).await;
                        } else {
                            let new_chat = NewChatModel {
                                name: None,
                                member_user_ids: chat.members.iter().map(|member| member.id.clone()).collect(),
                                first_message: message_str,
                            };
                            self.create_chat(new_chat).await;
                        }
                    }
                }
            }
        }
    }

    async fn open_chat(&mut self, chat: Chat) {
        let mut chat_id = None;
        if chat.id.is_some() {
            self.api_client.mark_chat_as_read(chat.id.unwrap()).await;
            self.main_window.chat_manager.load_chat(chat.internal_id.to_string());
            chat_id = Some(chat.id.unwrap().to_string());
        } else {
            // when searching for users they are shown as not existing chats
            // so if there IS a chat with this user, we should use its internal_id
            // it is a dirty temporary solution
            if let Some(existing_chat) = self.main_window.chat_manager.get_chat_by_name(&chat.name) {
                self.main_window.chat_manager.load_specifically_chat(
                    existing_chat.internal_id.to_string()
                );
                self.api_client.mark_chat_as_read(existing_chat.id.unwrap()).await;
                chat_id = Some(existing_chat.id.unwrap().to_string());
            } else {
                self.main_window.chat_manager.load_chat(
                    chat.internal_id.to_string()
                );
            };
        };
        self.main_window.set_active_input_entity(window::main::ActiveInputEntity::EnterMessage);
        if let Some(chat_id) = chat_id {
            self.main_window.chat_manager.clear_search_results();
            self.main_window.chat_manager.select_chat(chat_id);
        }
    }

    async fn run_search(&mut self) {
        let name_like = self.main_window.get_active_input();
        if name_like.is_empty() {
            self.main_window.chat_manager.clear_search_results();
            return;
        }

        match self.api_client.search_users(name_like.clone()).await {
            Ok(user_search_results) => {
                let mut chats = vec![];
                for user in user_search_results.users {
                    chats.push(Chat {
                        internal_id: user.username.clone(),
                        id: None,
                        name: user.username.clone(),
                        members: vec![user, self.user.as_ref().unwrap().clone()],
                        last_message: None,
                        number_of_unread_messages: 0,
                    });
                }

                self.main_window.chat_manager.set_search_results(chats);
            }
            Err(ApiError::Unauthenticated) => return,
            Err(e) => panic!("Error while searching for users: {:?}", e),
        }
    }

    pub async fn receive_message(&mut self) {
        if !self.api_client.is_authenticated() {
            return;
        }

        if let Some(message) = self.api_client.receive_message().await {
            if !self.main_window.chat_manager.has_chat(&message.chat_id) {
                match self.api_client.get_chat(message.chat_id).await {
                    Ok(chat_model) => {
                        self.main_window.chat_manager.add_chat(self.chat_builder.build_from_model(chat_model));
                    }
                    Err(ApiError::Unauthenticated) => {
                        return;
                    }
                    Err(e) => panic!("Error while loading chat: {:?}", e),
                }
            }
            self.main_window.chat_manager.add_message(message);
        }
    }

    async fn process_login(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        if res["username"].is_empty() || res["password"].is_empty() {
            self.login_window.login_error_message = "Username and password are required.".to_string();
            return;
        }
        let username = helpers::input_to_string(&res["username"]);
        let password = helpers::input_to_string(&res["password"]);

        match self.api_client.login(&username, &password).await {
            Ok(user_id) => {
                let user = User {
                    username,
                    id: user_id,
                };
                storage::store_user(&user);
                self.user = Some(user);

                let (chats_models, messages) = Self::load_chats_and_messages(&mut self.api_client).await;
                self.main_window.chat_manager.add_chats(self.chat_builder.build_from_models(chats_models));
                self.main_window.chat_manager.add_messages(messages);

                self.active_window = Windows::Main;
            }
            Err(e) => {
                self.login_window.login_error_message = e.to_string();
            }
        }
    }

    async fn process_register(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        if let Some(error) = self.validate_register_input(&res) {
            self.login_window.register_error_message = error;
            return;
        }
        let username = helpers::input_to_string(&res["username"]);
        let password = helpers::input_to_string(&res["password"]);

        match self.api_client.register(&username, &password).await {
            Ok(user_id) => {
                let user = User {
                    username,
                    id: user_id,
                };
                storage::store_user(&user);
                self.user = Some(user);
                self.active_window = Windows::Main;
            }
            Err(e) => {
                self.login_window.register_error_message = e.to_string();
            }
        }
    }

    fn validate_register_input(&self, input_values: &HashMap<String, TextInput>) -> Option<String> {
        // todo returning only one error message is temporary
        let mut error_message = String::new();

        if input_values["password"] != input_values["password_confirmation"] {
            error_message.push_str("Passwords do not match.\n");
            return Some(error_message);
        }

        if input_values["password"].len() < 8 {
            error_message.push_str("Password must be at least 8 characters long.\n");
            return Some(error_message);
        }

        if input_values["username"].len() < 3 {
            error_message.push_str("Username must be at least 3 characters long.\n");
            return Some(error_message);
        }

        None
    }

    async fn send_message(&mut self, message: NewMessage) {
        self.api_client.send_message(message).await;
    }

    async fn create_chat(&mut self, chat: NewChatModel) {
        match self.api_client.create_chat(chat).await {
            Ok(chat_model) => {
                let chat_id = chat_model.id.clone();

                // order is important: first clear search, then select chat
                self.main_window.chat_manager.clear_search_results();
                self.main_window.clear_search();
                self.main_window.chat_manager.add_chat(self.chat_builder.build_from_model(chat_model));
                self.main_window.chat_manager.select_chat(chat_id.to_string());
                self.main_window.chat_manager.load_chat(chat_id.to_string());
            }
            Err(ApiError::Unauthenticated) => return,
            Err(e) => panic!("Error while creating chat: {:?}", e),
        }
    }

    async fn load_chats_and_messages(api_client: &mut api::Client) -> (Vec<ChatModel>, HashMap<ChatId, Vec<Message>>) {
        match api_client.get_chats().await {
            Ok(chat_results) => {
                let mut messages = HashMap::new();
                for chat in chat_results.chats.iter() {
                    messages.insert(chat.id, chat.messages.clone());
                }
                let mut chats = Vec::new();
                for chat_model in chat_results.chats {
                    chats.push(chat_model);
                }

                (chats, messages)
            }
            Err(ApiError::Unauthenticated) => {
                (vec![], HashMap::new())
            }
            Err(e) => panic!("Error while loading chats: {:?}", e),
        }
    }
}

pub enum Windows {
    Login,
    Main,
}

pub fn get_current_user() -> Option<User> {
    storage::load_user()
}

fn extract_user_ids(chats: &Vec<Chat>) -> Vec<String> {
    let mut user_ids = HashSet::new();
    for chat in chats {
        for member in &chat.members {
            user_ids.insert(member.id.clone());
        }
    }
    user_ids.into_iter().collect()
}
