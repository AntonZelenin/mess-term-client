use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn process(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        // KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        // KeyCode::Down | KeyCode::Char('j') => app.chats.next(),
        // KeyCode::Up | KeyCode::Char('k') => app.chats.previous(),
        // KeyCode::Left | KeyCode::Char('h') => app.chats.unselect(),
        KeyCode::Enter => app.submit(),
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
        KeyCode::Tab => {
            app.switch_to_next_input();
        }
        // KeyCode::Esc => {
        //     app.switch_to_previous_input();
        // }
        _ => {}
    };
}
