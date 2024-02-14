use crossterm::event::{KeyCode, KeyEvent};
use crate::chat::manager::ChatManager;
use crate::helpers;
use crate::helpers::types::TextInput;
use crate::window::InputEntity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveInputEntity {
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
    pub chat_manager: ChatManager,
    search_input: TextInput,
    message_input: TextInput,
    active_input_entity: ActiveInputEntity,
    cursor_position: usize,
}

impl MainWindow {
    pub fn new(chat_manager: ChatManager) -> Self {
        Self {
            chat_manager,
            ..Default::default()
        }
    }
    pub fn get_active_input(&self) -> String {
        match self.active_input_entity {
            ActiveInputEntity::SearchChats => helpers::input_to_string(&self.search_input),
            ActiveInputEntity::EnterMessage => helpers::input_to_string(&self.message_input),
        }
    }

    pub fn get_active_input_mut(&mut self) -> &mut TextInput {
        match self.active_input_entity {
            ActiveInputEntity::SearchChats => &mut self.search_input,
            ActiveInputEntity::EnterMessage => &mut self.message_input,
        }
    }

    pub fn get_active_input_entity(&self) -> ActiveInputEntity {
        self.active_input_entity.clone()
    }

    pub fn get_search_input(&self) -> TextInput {
        self.search_input.clone()
    }

    pub fn get_message(&self) -> TextInput {
        self.message_input.clone()
    }

    pub fn pop_message_input(&mut self) -> TextInput {
        let message = self.message_input.clone();
        self.message_input.clear();
        self.reset_cursor();
        message
    }

    pub fn set_active_input_entity(&mut self, active_input_entity: ActiveInputEntity) {
        self.active_input_entity = active_input_entity;
        self.reset_cursor();
    }

    pub fn clear_search(&mut self) {
        self.search_input.clear();
        self.reset_cursor();
    }

    fn move_chat_cursor_up(&mut self) {
        self.chat_manager.select_previous_chat();
    }

    fn move_chat_cursor_down(&mut self) {
        self.chat_manager.select_next_chat();
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
            KeyCode::Up => {
                if self.chat_manager.get_loaded_chat().is_none() {
                    self.move_chat_cursor_up();
                }
            }
            KeyCode::Down => {
                if self.chat_manager.get_loaded_chat().is_none() {
                    self.move_chat_cursor_down();
                }
            }
            KeyCode::Esc => {
                if self.chat_manager.get_loaded_chat().is_some() {
                    self.chat_manager.unload_chat();
                    self.message_input.clear();
                    self.reset_cursor();
                    self.set_active_input_entity(ActiveInputEntity::SearchChats);
                } else if self.chat_manager.get_selected_chat().is_some() {
                    self.chat_manager.unselect_chat();
                } else {
                    // todo method?
                    self.chat_manager.clear_search_results();
                    self.search_input.clear();
                    self.reset_cursor();
                }
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
        // todo duplicate code!!!
        if self.cursor_position == 0 {
            return;
        }

        let cursor_position = self.cursor_position - 1;
        self.get_active_input_mut().remove(cursor_position);
        self.move_cursor_left();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
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
