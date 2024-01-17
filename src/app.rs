#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub chats: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn add_chats(&mut self, chats: Vec<String>) {
        self.chats.extend(chats);
    }
}
