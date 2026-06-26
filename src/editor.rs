use crate::{Buffer, Cursor};

use crossterm::event::KeyCode;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    mode: EditorMode,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Self {
        Editor {
            buffer,
            cursor: Cursor::default(),
            mode: EditorMode::Normal,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }

    pub fn handle_keypress(&mut self, key: KeyCode) -> Option<EditorAction> {
        match self.mode {
            EditorMode::Normal => self.handle_normal_mode_keypress(key),
            EditorMode::Insert => self.handle_insert_mode_keypress(key),
        }
    }

    fn handle_normal_mode_keypress(&mut self, key: KeyCode) -> Option<EditorAction> {
        match key {
            KeyCode::Char('q') => Some(EditorAction::Quit),
            KeyCode::Char('i') => {
                self.mode = EditorMode::Insert;
                None
            }
            KeyCode::Char('h') => {
                self.move_cursor_left();
                None
            }
            KeyCode::Char('j') => {
                self.move_cursor_down();
                None
            }
            KeyCode::Char('k') => {
                self.move_cursor_up();
                None
            }
            KeyCode::Char('l') => {
                self.move_cursor_right();
                None
            }
            _ => None,
        }
    }

    fn handle_insert_mode_keypress(&mut self, key: KeyCode) -> Option<EditorAction> {
        match key {
            KeyCode::Esc => {
                self.mode = EditorMode::Normal;
                None
            }
            KeyCode::Char(c) => {
                self.insert_char(c);
                None
            }
            KeyCode::Backspace => {
                self.backspace();
                None
            }
            _ => None,
        }
    }

    fn move_cursor_up(&mut self) {
        self.cursor.up(&self.buffer);
    }

    fn move_cursor_down(&mut self) {
        self.cursor.down(&self.buffer);
    }

    fn move_cursor_left(&mut self) {
        self.cursor.left();
    }

    fn move_cursor_right(&mut self) {
        self.cursor.right(&self.buffer);
    }

    fn insert_char(&mut self, c: char) {
        self.buffer
            .insert_at_position(c, self.cursor.row(), self.cursor.col());

        self.cursor.right(&self.buffer);
    }

    fn backspace(&mut self) {
        if self.cursor.col() == 0 {
            return;
        }

        self.buffer
            .remove_at_position(self.cursor.row(), self.cursor.col() - 1);

        self.cursor.left();
    }
}

#[derive(Debug, PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
}

#[derive(Debug, PartialEq)]
pub enum EditorAction {
    Quit,
}

#[cfg(test)]
mod normal_mode_key_tests {
    use super::*;

    #[test]
    fn test_editor_handles_quit() {
        let mut editor = Editor::new(Buffer::new(""));

        let action = editor.handle_keypress(KeyCode::Char('q'));

        assert_eq!(action, Some(EditorAction::Quit));
    }

    #[test]
    fn test_editor_handles_left() {
        let mut editor = Editor::new(Buffer::new("hello\nworld"));
        editor.handle_keypress(KeyCode::Char('l'));
        editor.handle_keypress(KeyCode::Char('h'));
        assert_eq!(editor.cursor().col(), 0);
    }

    #[test]
    fn test_editor_handles_down() {
        let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
        editor.handle_keypress(KeyCode::Char('j'));
        assert_eq!(editor.cursor().row(), 1);
    }

    #[test]
    fn test_editor_handles_up() {
        let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
        editor.handle_keypress(KeyCode::Char('j'));
        editor.handle_keypress(KeyCode::Char('k'));
        assert_eq!(editor.cursor().row(), 0);
    }

    #[test]
    fn test_editor_handles_right() {
        let mut editor = Editor::new(Buffer::new("hello\nworld"));
        editor.handle_keypress(KeyCode::Char('l'));
        assert_eq!(editor.cursor().col(), 1);
    }

    #[test]
    fn test_editor_handle_switching_to_insert_mode() {
        let mut editor = Editor::new(Buffer::new(""));

        editor.handle_keypress(KeyCode::Char('i'));
        assert_eq!(editor.mode(), &EditorMode::Insert);
    }
}

#[cfg(test)]
mod insert_mode_key_tests {
    use super::*;

    #[test]
    fn test_editor_handles_return_to_normal_mode() {
        let mut editor = Editor::new(Buffer::new(""));

        editor.handle_keypress(KeyCode::Char('i'));
        editor.handle_keypress(KeyCode::Esc);

        assert_eq!(editor.mode(), &EditorMode::Normal);
    }

    #[test]
    fn test_editor_inserts_normal_characters() {
        let mut editor = Editor::new(Buffer::new("Hello"));

        // switch to insert mode
        editor.handle_keypress(KeyCode::Char('i'));

        editor.handle_keypress(KeyCode::Char('h'));
        editor.handle_keypress(KeyCode::Char('j'));
        editor.handle_keypress(KeyCode::Char('k'));
        editor.handle_keypress(KeyCode::Char('l'));
        editor.handle_keypress(KeyCode::Char('i'));
        editor.handle_keypress(KeyCode::Char('q'));

        assert_eq!(editor.buffer().to_string(), "hjkliqHello");
    }

    #[test]
    fn test_editor_removes_character_at_backspace() {
        let mut editor = Editor::new(Buffer::new("Seppuku"));

        // move right
        editor.handle_keypress(KeyCode::Char('l'));

        // switch to insert mode
        editor.handle_keypress(KeyCode::Char('i'));

        editor.handle_keypress(KeyCode::Backspace);

        assert_eq!(editor.buffer().to_string(), "eppuku");
    }

    #[test]
    fn test_cursor_moves_back_after_backspace() {
        let mut editor = Editor::new(Buffer::new("Seppuku"));

        // move right
        editor.handle_keypress(KeyCode::Char('l'));

        // switch to insert mode
        editor.handle_keypress(KeyCode::Char('i'));

        editor.handle_keypress(KeyCode::Backspace);

        assert_eq!(editor.cursor().col(), 0);
    }
}
