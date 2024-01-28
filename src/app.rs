use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::api;
use crate::chat::Chat;
use crate::contact::Contact;
use crate::helpers::list::StatefulList;
use crate::session;
use crate::ui::states::AuthWindowState;
use crate::ui::chat::StatefulChat;

pub struct App {
    pub chats: StatefulList<StatefulChat>,
    pub contacts: Vec<Contact>,
    pub auth_window: AuthWindowState,
    pub active_input_state: InputStates,
    api_client: api::Client,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut api_client = api::Client::new(session::load_session());
        let mut stateful_chats = StatefulList::new();
        let mut contacts = Vec::new();

        if api_client.is_authenticated() {
            let chats = api_client.get_chats().unwrap();

            stateful_chats = StatefulList::with_items(
                chats
                    .into_iter()
                    .map(StatefulChat::from_chat)
                    .collect(),
            );
            contacts = api_client.get_contacts().unwrap();
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
}

pub enum InputStates {
    Login,
    SearchChat,
    EnterMessage,
}

pub fn process_input(app: &mut App) {
    if let Event::Key(key) = event::read().expect("Failed to read key event") {
        if key.kind == KeyEventKind::Press {
            match key.code {
                // KeyCode::Enter => app.submit(),
                KeyCode::Char(to_insert) => {
                    app.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    app.delete_char();
                }
                KeyCode::Left => {
                    app.move_cursor_left();
                }
                KeyCode::Right => {
                    app.move_cursor_right();
                }
                // KeyCode::Esc => {
                //     app.switch_to_previous_input();
                // }
                _ => {}
            }
        }
    }
}

pub trait InputEntity {
    fn enter_char(&mut self, new_char: char);
    fn delete_char(&mut self);
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize;
    fn reset_cursor(&mut self);
    fn switch_to_next_input(&mut self);
}