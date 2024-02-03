use std::collections::HashMap;
use crossterm::event::KeyEvent;
use crate::{api, auth};
use crate::chat::Chat;
use crate::contact::Contact;
use crate::helpers::list::StatefulList;
use crate::input::entities::login::{LoginWindowEntity, LoginTabs};
use crate::input::entities::search::SearchInputEntity;
use crate::input::InputEntity;
use crate::ui::chat::StatefulChat;

pub struct App {
    pub chats: StatefulList<StatefulChat>,
    pub contacts: HashMap<String, Contact>,
    pub login_window: LoginWindowEntity,
    pub search_input: SearchInputEntity,
    pub active_input_state: InputStates,
    api_client: api::Client,
    should_quit: bool,
}

impl App {
    pub fn new(
        mut api_client: api::Client,
    ) -> Self {
        let mut stateful_chats = StatefulList::new();
        let mut contacts = HashMap::new();

        if api_client.is_authenticated() {
            // contacts = Self::load_contacts(&mut api_client);
            stateful_chats = Self::load_chats(&mut api_client);
        }

        Self {
            chats: stateful_chats,
            contacts,
            login_window: LoginWindowEntity::default(),
            search_input: SearchInputEntity::default(),
            active_input_state: if !api_client.is_authenticated() { InputStates::Login } else { InputStates::SearchChat },
            api_client,
            should_quit: false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_client.is_authenticated()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.chats.push(StatefulChat::from_chat(chat));
    }

    fn get_active_input_state(&mut self) -> &mut dyn InputEntity {
        match self.active_input_state {
            InputStates::Login => &mut self.login_window,
            InputStates::SearchChat => &mut self.search_input,
            InputStates::EnterMessage => unimplemented!(),
        }
    }

    pub fn pass_input_to_active_entity(&mut self, key_event: KeyEvent) {
        self.get_active_input_state().process_input(key_event);
    }

    pub fn submit(&mut self) {
        match self.active_input_state {
            InputStates::Login => {
                match self.login_window.active_tab {
                    LoginTabs::Login => {
                        self.process_login();
                    }
                    LoginTabs::Register => {
                        self.process_register();
                    }
                }
            }
            InputStates::SearchChat => {
                let search_query = self.search_input.input.clone();
                let chats = self.api_client.search_chats(search_query).unwrap();

                self.chats = StatefulList::with_items(
                    chats
                        .into_iter()
                        .map(StatefulChat::from_chat)
                        .collect(),
                );
            }
            InputStates::EnterMessage => unimplemented!(),
        }
    }

    fn process_login(&mut self) {
        let res = self.login_window.get_input_values();

        match self.api_client.login(&res["username"], &res["password"]) {
            Ok(_) => {
                auth::store_auth_tokens(&self.api_client.get_auth_tokens().expect("Tried to save session, but it's None"));

                // self.contacts = Self::load_contacts(&mut self.api_client);
                // self.chats = Self::load_chats(&mut self.api_client);

                self.active_input_state = InputStates::SearchChat;
            }
            Err(e) => {
                self.login_window.login_error_message = e;
            }
        }
    }

    fn process_register(&mut self) {
        let res = self.login_window.get_input_values();

        let error = self.validate_register_input(&res);
        if !error.is_empty() {
            self.login_window.register_error_message = error;
            return;
        }

        match self.api_client.register(&res["username"], &res["password"]) {
            Ok(_) => {
                auth::store_auth_tokens(&self.api_client.get_auth_tokens().expect("Tried to save session, but it's None"));

                // self.contacts = Self::load_contacts(&mut self.api_client);
                // self.chats = Self::load_chats(&mut self.api_client);

                self.active_input_state = InputStates::SearchChat;
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

    pub fn load_chats(api_client: &mut api::Client) -> StatefulList<StatefulChat> {
        let chats = api_client.get_chats().unwrap();
        StatefulList::with_items(
            chats
                .into_iter()
                .map(StatefulChat::from_chat)
                .collect(),
        )
    }

    pub fn load_contacts(api_client: &mut api::Client) -> HashMap<String, Contact> {
        api_client.get_contacts().unwrap()
    }
}

pub enum InputStates {
    Login,
    SearchChat,
    EnterMessage,
}
