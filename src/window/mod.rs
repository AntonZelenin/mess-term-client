use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub mod login;
pub mod main;

pub trait InputEntity {
    fn process_input(&mut self, key_event: KeyEvent);
    fn enter_char(&mut self, new_char: char);
    fn delete_char(&mut self);
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize;
    fn reset_cursor(&mut self);
    fn switch_to_next_input(&mut self);
    // todo this trait needs to take into account all possible inputs, bad design
    fn switch_tabs(&mut self);
}

pub fn process(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        // KeyCode::Down | KeyCode::Char('j') => app.chats.next(),
        // KeyCode::Up | KeyCode::Char('k') => app.chats.previous(),
        // KeyCode::Left | KeyCode::Char('h') => app.chats.unselect(),
        KeyCode::Enter => app.submit(),
        // KeyCode::Esc => {
        //     app.switch_to_previous_input();
        // }
        _ => {
            app.pass_input_to_active_entity(key_event);
        }
    };
}
