use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent};
use crate::helpers::types::TextInput;
use crate::window::InputEntity;

#[derive(Debug, Clone, Copy)]
pub enum LoginActiveInput {
    Username,
    Password,
    RegisterUsername,
    RegisterPassword,
    RegisterPasswordConfirmation,
}

#[derive(Debug, Clone, Copy)]
pub enum LoginTabs {
    Login,
    Register,
}

// todo why all this in states? Bad place, bad name
pub struct LoginWindow {
    // Login
    pub username_input: TextInput,
    pub password_input: TextInput,
    pub login_error_message: String,

    // Register
    pub register_username_input: TextInput,
    pub register_password_input: TextInput,
    pub register_password_confirmation_input: TextInput,
    pub register_error_message: String,

    pub active_input_field: LoginActiveInput,
    pub active_tab: LoginTabs,
    cursor_position: usize,

    actual_password_input: TextInput,
    actual_register_password_input: TextInput,
    actual_register_password_confirmation_input: TextInput,
}

impl Default for LoginWindow {
    fn default() -> Self {
        Self {
            username_input: TextInput::new(),
            password_input: TextInput::new(),
            login_error_message: String::new(),

            register_username_input: TextInput::new(),
            register_password_input: TextInput::new(),
            register_password_confirmation_input: TextInput::new(),
            register_error_message: String::new(),

            active_input_field: LoginActiveInput::Username,
            active_tab: LoginTabs::Login,
            cursor_position: 0,
            actual_password_input: TextInput::new(),
            actual_register_password_input: TextInput::new(),
            actual_register_password_confirmation_input: TextInput::new(),
        }
    }
}

impl InputEntity for LoginWindow {
    fn process_input(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(to_insert) => {
                self.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                self.delete_char();
            }
            KeyCode::Left | KeyCode::Right => {
                // self.move_cursor_left();
                self.switch_tabs()
            }
            // KeyCode::Right => {
            // self.move_cursor_right();
            // }
            KeyCode::Tab => {
                self.switch_to_next_input();
            }
            _ => {}
        };
    }

    fn enter_char(&mut self, new_char: char) {
        let cursor_position = self.cursor_position;
        match self.active_input_field {
            LoginActiveInput::Username | LoginActiveInput::RegisterUsername => {
                self.get_active_ui_input_mut().insert(cursor_position, new_char);
            }
            LoginActiveInput::Password | LoginActiveInput::RegisterPassword | LoginActiveInput::RegisterPasswordConfirmation => {
                self.get_active_input_mut().insert(cursor_position, new_char);
                self.get_active_ui_input_mut().insert(cursor_position, '*');
            }
        }

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position == 0 {
            return;
        }

        let cursor_position = self.cursor_position - 1;
        match self.active_input_field {
            LoginActiveInput::Password | LoginActiveInput::RegisterPassword | LoginActiveInput::RegisterPasswordConfirmation => {
                self.get_active_input_mut().remove(cursor_position);
                self.get_active_ui_input_mut().remove(cursor_position);
            }
            LoginActiveInput::Username | LoginActiveInput::RegisterUsername => {
                self.get_active_ui_input_mut().remove(cursor_position);
            }
        }
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

    fn switch_to_next_input(&mut self) {
        self.active_input_field = match self.active_input_field {
            LoginActiveInput::Username => LoginActiveInput::Password,
            LoginActiveInput::Password => LoginActiveInput::Username,

            LoginActiveInput::RegisterUsername => LoginActiveInput::RegisterPassword,
            LoginActiveInput::RegisterPassword => LoginActiveInput::RegisterPasswordConfirmation,
            LoginActiveInput::RegisterPasswordConfirmation => LoginActiveInput::RegisterUsername,
        };
        self.move_cursor_to_eol();
    }

    fn switch_tabs(&mut self) {
        match self.active_tab {
            LoginTabs::Login => {
                self.active_tab = LoginTabs::Register;
                self.active_input_field = LoginActiveInput::RegisterUsername;
            }
            LoginTabs::Register => {
                self.active_tab = LoginTabs::Login;
                self.active_input_field = LoginActiveInput::Username;
            }
        };
        self.move_cursor_to_eol();
    }
}

impl LoginWindow {
    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn get_input_values(&self) -> HashMap<String, TextInput> {
        let mut input_values = HashMap::new();
        match self.active_tab {
            LoginTabs::Login => {
                input_values.insert("username".to_string(), self.username_input.clone());
                input_values.insert("password".to_string(), self.actual_password_input.clone());
                input_values
            }
            LoginTabs::Register => {
                input_values.insert("username".to_string(), self.register_username_input.clone());
                input_values.insert("password".to_string(), self.actual_register_password_input.clone());
                input_values.insert("password_confirmation".to_string(), self.actual_register_password_confirmation_input.clone());
                input_values
            }
        }
    }

    fn get_active_ui_input_mut(&mut self) -> &mut TextInput {
        match self.active_input_field {
            LoginActiveInput::Username => &mut self.username_input,
            LoginActiveInput::Password => &mut self.password_input,
            LoginActiveInput::RegisterUsername => &mut self.register_username_input,
            LoginActiveInput::RegisterPassword => &mut self.register_password_input,
            LoginActiveInput::RegisterPasswordConfirmation => &mut self.register_password_confirmation_input,
        }
    }

    fn get_active_input(&self) -> TextInput {
        match self.active_input_field {
            LoginActiveInput::Username => self.username_input.clone(),
            LoginActiveInput::Password => self.actual_password_input.clone(),
            LoginActiveInput::RegisterUsername => self.register_username_input.clone(),
            LoginActiveInput::RegisterPassword => self.actual_register_password_input.clone(),
            LoginActiveInput::RegisterPasswordConfirmation => self.actual_register_password_confirmation_input.clone(),
        }
    }

    fn get_active_input_mut(&mut self) -> &mut TextInput {
        match self.active_input_field {
            LoginActiveInput::Username => &mut self.username_input,
            LoginActiveInput::Password => &mut self.actual_password_input,
            LoginActiveInput::RegisterUsername => &mut self.register_username_input,
            LoginActiveInput::RegisterPassword => &mut self.actual_register_password_input,
            LoginActiveInput::RegisterPasswordConfirmation => &mut self.actual_register_password_confirmation_input,
        }
    }

    fn move_cursor_to_eol(&mut self) {
        self.cursor_position = self.get_active_input().len();
    }
}
