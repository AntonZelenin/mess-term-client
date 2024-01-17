use crate::chat::Chat;
use crate::helpers::list::StatefulList;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub chats: StatefulList<Chat>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
