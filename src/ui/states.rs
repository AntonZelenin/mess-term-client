pub struct AuthWindowState {
    pub username_input: String,
    pub password_input: String,
    cursor_position: usize,
    pub active_input: AuthActiveInput,
}

impl Default for AuthWindowState {
    fn default() -> Self {
        Self {
            username_input: String::new(),
            password_input: String::new(),
            cursor_position: 0,
            active_input: AuthActiveInput::Username,
        }
    }
}

pub enum AuthActiveInput {
    Username,
    Password,
}

impl AuthWindowState {
    fn get_active_input_mut(&mut self) -> &mut String {
        match self.active_input {
            AuthActiveInput::Username => &mut self.username_input,
            AuthActiveInput::Password => &mut self.password_input,
        }
    }

    fn get_active_input(&self) -> &String {
        match self.active_input {
            AuthActiveInput::Username => &self.username_input,
            AuthActiveInput::Password => &self.password_input,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
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
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.get_active_input().chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.get_active_input().chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            *self.get_active_input_mut() = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.get_active_input().len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}