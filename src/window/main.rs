use crossterm::event::{KeyCode, KeyEvent};
use crate::chat::Chat;
use crate::helpers::list::StatefulList;
use crate::ui::chat::StatefulChat;
use crate::window::InputEntity;

#[derive(Debug, Clone, Copy)]
pub enum  ActiveInputEntity {
    SearchChats,
    EnterMessage,
}

impl Default for ActiveInputEntity {
    fn default() -> Self {
        ActiveInputEntity::SearchChats
    }
}

#[derive(Default)]
pub struct MainWindow {
    chats: StatefulList<StatefulChat>,
    search_results: StatefulList<StatefulChat>,
    search_input: String,
    enter_message_input: String,
    active_input_entity: ActiveInputEntity,
    cursor_position: usize,
}

impl MainWindow {
    pub fn new(chats: Vec<Chat>) -> Self {
        let mut main_window = MainWindow::default();
        main_window.add_chats(chats);
        main_window
    }

    pub fn add_chats(&mut self, chats: Vec<Chat>) {
        for chat in chats {
            self.chats.items.push(StatefulChat::from_chat(chat));
        }
    }

    pub fn get_active_input(&self) -> String {
        match self.active_input_entity {
            ActiveInputEntity::SearchChats => self.search_input.clone().to_string(),
            ActiveInputEntity::EnterMessage => self.enter_message_input.clone().to_string(),
        }
    }

    pub fn get_active_input_mut(&mut self) -> &mut String {
        match self.active_input_entity {
            ActiveInputEntity::SearchChats => &mut self.search_input,
            ActiveInputEntity::EnterMessage => &mut self.enter_message_input,
        }
    }

    pub fn get_active_input_entity(&self) -> ActiveInputEntity {
        self.active_input_entity.clone()
    }

    pub fn set_search_results(&mut self, chats: Vec<Chat>) {
        self.search_results.items.clear();
        for chat in chats {
            self.search_results.items.push(StatefulChat::from_chat(chat));
        }
    }

    pub fn get_search_input(&self) -> String {
        self.search_input.clone()
    }

    pub fn get_chats(&self) -> &StatefulList<StatefulChat> {
        if self.search_input.is_empty() {
            &self.chats
        } else {
            &self.search_results
        }
    }

    pub fn get_chats_mut(&mut self) -> &mut StatefulList<StatefulChat> {
        if self.search_input.is_empty() {
            &mut self.chats
        } else {
            &mut self.search_results
        }
    }
}

impl InputEntity for MainWindow {
    fn process_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right();
            }
            // KeyCode::Tab => {
            //     self.switch_to_next_input();
            // }
            _ => {}
        };
    }

    fn enter_char(&mut self, new_char: char) {
        let cursor_position = self.cursor_position;
        self.get_active_input_mut().insert(cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 0;

            let active_input = self.get_active_input();
            // Getting all characters before the selected character.
            let before_char_to_delete = active_input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = active_input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            *self.get_active_input_mut() = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(0);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(0);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.get_active_input().len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn switch_to_next_input(&mut self) {}

    fn switch_tabs(&mut self) {}
}