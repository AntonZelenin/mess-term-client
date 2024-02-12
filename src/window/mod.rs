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

pub async fn process(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        // yes yes, I know, it's a hack and it'll work only for 3 languages: eng, ukr and rus
        KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('с') | KeyCode::Char('С') | KeyCode::Char('с') | KeyCode::Char('С') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            } else {
                app.pass_input_to_active_entity(key_event);
            }
        }
        KeyCode::Enter => app.submit().await,
        _ => {
            app.pass_input_to_active_entity(key_event);
        }
    };
}
