use std::collections::HashMap;
use crate::{api, auth};
use crate::chat::Chat;
use crate::contact::Contact;
use crate::helpers::list::StatefulList;
use crate::input::InputEntity;
use crate::ui::states::AuthWindowState;
use crate::ui::chat::StatefulChat;

pub struct App {
    pub chats: StatefulList<StatefulChat>,
    pub contacts: HashMap<String, Contact>,
    pub auth_window: AuthWindowState,
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
            stateful_chats = Self::load_chats(&mut api_client);
            contacts = Self::load_contacts(&mut api_client);
        }

        Self {
            chats: stateful_chats,
            contacts,
            auth_window: AuthWindowState::default(),
            active_input_state: if !api_client.is_authenticated() { InputStates::Login } else { InputStates::SearchChat },
            api_client,
            should_quit: false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_client.is_authenticated()
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
        self.api_client.login(username, password)
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

    fn get_active_input_state<'a>(&'a mut self) -> &'a mut dyn InputEntity {
        match self.active_input_state {
            InputStates::Login => &mut self.auth_window,
            InputStates::SearchChat => unimplemented!(),
            InputStates::EnterMessage => unimplemented!(),
        }
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.get_active_input_state().enter_char(new_char);
    }

    pub fn delete_char(&mut self) {
        self.get_active_input_state().delete_char();
    }

    pub fn move_cursor_left(&mut self) {
        self.get_active_input_state().move_cursor_left();
    }

    pub fn move_cursor_right(&mut self) {
        self.get_active_input_state().move_cursor_right();
    }

    pub fn switch_to_next_input(&mut self) {
        self.get_active_input_state().switch_to_next_input();
    }

    pub fn submit(&mut self) {
        match self.active_input_state {
            InputStates::Login => {
                let res = self.auth_window.get_input_values();
                // todo handle incorrect username/password
                self.login(&res["username"], &res["password"]).expect("Failed to login");
                auth::store_auth_tokens(&self.api_client.get_auth_tokens().expect("Tried to save session, but it's None"));
                self.chats = Self::load_chats(&mut self.api_client);
                self.contacts = Self::load_contacts(&mut self.api_client);
                self.active_input_state = InputStates::SearchChat;
            },
            InputStates::SearchChat => unimplemented!(),
            InputStates::EnterMessage => unimplemented!(),
        }
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
