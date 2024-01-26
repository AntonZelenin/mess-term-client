use crate::helpers::list::StatefulList;
use crate::session::Session;

#[derive(Debug, Clone)]
pub struct Chat {
    pub id: u32,
    pub name: String,
    pub messages: Option<StatefulList<String>>,
}

impl Chat {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            messages: None,
        }
    }

    pub fn add_messages(&mut self, messages: Vec<String>) {
        if let Some(m) = &mut self.messages {
            m.items.extend(messages);
        } else {
            self.messages = Some(StatefulList::with_items(messages));
        }
    }
}

pub fn load_chats(session: Session) -> Vec<Chat> {
    unimplemented!()
}
