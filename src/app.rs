use std::collections::HashMap;
use crossterm::event::KeyEvent;
use crate::{api, window};
use crate::chat::{Chat, Message};
use crate::contact::Contact;
use crate::window::InputEntity;
use crate::window::login::{LoginTabs, LoginWindow};
use crate::window::main::MainWindow;

pub struct App {
    pub login_window: LoginWindow,
    pub main_window: MainWindow,
    active_window: Windows,
    contacts: HashMap<String, Contact>,
    api_client: api::Client,
    should_quit: bool,
}

impl App {
    pub fn new(
        mut api_client: api::Client,
    ) -> Self {
        let mut chats = Vec::new();
        let mut contacts = HashMap::new();

        if api_client.is_authenticated() {
            // contacts = Self::load_contacts(&mut api_client);
            chats = Self::load_chats(&mut api_client);
        }

        Self {
            login_window: LoginWindow::default(),
            main_window: MainWindow::new(chats),
            active_window: if !api_client.is_authenticated() { Windows::Login } else { Windows::Main },
            contacts,
            api_client,
            should_quit: false,
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

    pub fn submit(&mut self) {
        match self.active_window {
            Windows::Login => {
                match self.login_window.active_tab {
                    LoginTabs::Login => {
                        self.process_login();
                    }
                    LoginTabs::Register => {
                        self.process_register();
                    }
                }
            }
            Windows::Main => {
                match self.main_window.get_active_input_entity() {
                    window::main::ActiveInputEntity::SearchChats => {
                        if self.main_window.get_chats().state.selected().is_some() {
                            self.main_window.load_chat();
                            self.main_window.set_active_input_entity(window::main::ActiveInputEntity::EnterMessage);
                        } else {
                            self.main_window.set_search_results(
                                self.api_client.search_users(self.main_window.get_active_input()).unwrap()
                            );
                        }
                    }
                    window::main::ActiveInputEntity::EnterMessage => {
                        self.send_message(self.main_window.pop_message_input());
                    }
                }
            }
        }
    }

    fn process_login(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        match self.api_client.login(&res["username"], &res["password"]) {
            Ok(_) => {
                // self.contacts = Self::load_contacts(&mut self.api_client);
                self.main_window.add_chats(Self::load_chats(&mut self.api_client));

                self.active_window = Windows::Main;
            }
            Err(e) => {
                self.login_window.login_error_message = e;
            }
        }
    }

    fn process_register(&mut self) {
        self.login_window.login_error_message = String::new();
        let res = self.login_window.get_input_values();

        let error = self.validate_register_input(&res);
        if !error.is_empty() {
            self.login_window.register_error_message = error;
            return;
        }

        match self.api_client.register(&res["username"], &res["password"]) {
            Ok(_) => {
                // self.contacts = Self::load_contacts(&mut self.api_client);
                self.main_window.add_chats(Self::load_chats(&mut self.api_client));

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

    pub fn load_chats(api_client: &mut api::Client) -> Vec<Chat> {
        api_client.get_chats().unwrap()
    }

    pub fn load_contacts(api_client: &mut api::Client) -> HashMap<String, Contact> {
        api_client.get_contacts().unwrap()
    }

    fn send_message(&self, message: Message) {
        self.api_client.send_message(&message);
    }
}

pub enum Windows {
    Login,
    Main,
}
