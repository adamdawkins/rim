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

    pub fn handle_keypress(&mut self, key: KeyCode) -> Option<EditorAction> {
        match key {
            KeyCode::Char('q') => Some(EditorAction::Quit),
            _ => None,
        }
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor.up(&self.buffer);
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor.down(&self.buffer);
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor.left();
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor.right(&self.buffer);
    }
}

enum EditorMode {
    Normal,
    // Insert,
}

#[derive(Debug, PartialEq)]
pub enum EditorAction {
    Quit,
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_editor_handles_quit() {
        let mut editor = Editor::new(Buffer::new(""));

        let action = editor.handle_keypress(KeyCode::Char('q'));

        assert_eq!(action, Some(EditorAction::Quit));
    }
}
