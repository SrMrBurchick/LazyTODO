use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{events::{DatabaseResponse, HistoryRequest, Request, Response}, ui::{components::base::component::Component, sections::ESectionId}};

#[derive(Debug, Clone)]
pub enum EInputMode {
    Normal,
    Editing,
}

#[derive(Debug, Clone)]
pub struct UserInputComponent {
    mode: EInputMode,
    data: String,
    cursor_position: usize
}

impl Default for UserInputComponent {
    fn default() -> Self {
        UserInputComponent::new()
    }
}

impl UserInputComponent {
    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn get_mode(&self) -> EInputMode {
        self.mode.clone()
    }

    fn confirm(&mut self) {
        self.mode = EInputMode::Normal;
    }

    fn update_data(&mut self, data: char) {
        let index = self.byte_index();
        self.data.insert(index, data);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.data
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_position)
            .unwrap_or(self.data.len())
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
        new_cursor_pos.clamp(0, self.data.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.cursor_position = 0;
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
            let before_char_to_delete = self.data.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.data.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.data = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

}

impl Component for UserInputComponent {
    fn new() -> Self {
        UserInputComponent {
            mode: EInputMode::Normal,
            data: String::default(),
            cursor_position: 0
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Request, String> {
        match self.mode {
            EInputMode::Normal => {
                match key.code {
                    KeyCode::Char('i') => {
                        self.mode = EInputMode::Editing;
                    },
                    _ => {
                    }
                }
            },
            EInputMode::Editing => {
                match key.code {
                    KeyCode::Enter => self.confirm(),
                    KeyCode::Esc => {
                        self.mode = EInputMode::Normal;
                    }
                    KeyCode::Char(data) => self.update_data(data),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    _ => {
                    }
                }
            }
        }

        Ok(Request::Nothing)
    }

    fn handle_response(&mut self, response: Response) {
        match response {
            _ => {
            }
        }
    }
}
