use crate::chat::Chat;
use crate::helpers::list::StatefulList;
use crate::session::Session;

#[derive(Default)]
pub struct App {
    should_quit: bool,
    pub chats: StatefulList<Chat>,
    session: Option<Session>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_active_session(&self) -> bool {
        self.session.is_some()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn add_chat(&mut self, chat: Chat) {
        self.chats.items.push(chat);
    }

    pub fn add_chats(&mut self, chats: Vec<Chat>) {
        self.chats.items.extend(chats);
    }
}
