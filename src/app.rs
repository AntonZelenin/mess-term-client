use crate::helpers::list::StatefulList;

#[derive(Debug, Default)]
pub struct App<'a> {
    pub should_quit: bool,
    pub chats: StatefulList<&'a str>,
    pub messages: Option<StatefulList<&'a str>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
