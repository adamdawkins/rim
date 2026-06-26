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
            // Commands
            KeyCode::Char('q') => Some(EditorAction::Quit),
            KeyCode::Char('i') => {
                self.mode = EditorMode::Insert;
                None
            }

            // Motions
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
            KeyCode::Char('0') => {
                self.move_cursor_to_start_of_line(self.cursor.row());
                None
            }
            KeyCode::Char('$') => {
                self.move_cursor_to_end_of_line(self.cursor.row());
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
            KeyCode::Backspace => {
                self.backspace();
                None
            }
            KeyCode::Enter => {
                self.enter();
                None
            }
            KeyCode::Char(c) => {
                self.insert_char(c);
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

    fn move_cursor_to_end_of_line(&mut self, row: usize) {
        let col = self.buffer.max_col(row);
        self.cursor.move_to_col(col);
    }

    fn move_cursor_to_start_of_line(&mut self, row: usize) {
        self.cursor.move_to(row, 0);
    }

    fn insert_char(&mut self, c: char) {
        self.buffer
            .insert_at_position(c, self.cursor.row(), self.cursor.col());

        self.cursor.right(&self.buffer);
    }

    fn enter(&mut self) {
        let row = self.cursor.row();
        let col = self.cursor.col();

        self.buffer.split_line(row, col);
        self.move_cursor_to_start_of_line(row + 1);
    }

    fn backspace(&mut self) {
        match (self.cursor.row(), self.cursor.col()) {
            (0, 0) => return,
            (_, 0) => {
                self.backspace_at_start_of_line();
            }
            _ => {
                self.backspace_char();
            }
        }
    }

    fn backspace_at_start_of_line(&mut self) {
        let row = self.cursor.row();
        // max_col is the index of the end of the last line,
        // we add one so that the cursor is placed at the end of the previous line after
        // joining
        let previous_line_length = self.buffer.max_col(row - 1) + 1;
        self.buffer.join_lines(row);
        self.cursor.move_to(row - 1, previous_line_length);
    }

    fn backspace_char(&mut self) {
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
mod tests {
    use super::*;

    mod normal_mode {
        use super::*;

        mod motions {
            use super::*;

            #[test]
            fn left() {
                let mut editor = Editor::new(Buffer::new("hello\nworld"));
                editor.handle_keypress(KeyCode::Char('l'));
                editor.handle_keypress(KeyCode::Char('h'));
                assert_eq!(editor.cursor().col(), 0);
            }

            #[test]
            fn down() {
                let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
                editor.handle_keypress(KeyCode::Char('j'));
                assert_eq!(editor.cursor().row(), 1);
            }

            #[test]
            fn up() {
                let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
                editor.handle_keypress(KeyCode::Char('j'));
                editor.handle_keypress(KeyCode::Char('k'));
                assert_eq!(editor.cursor().row(), 0);
            }

            #[test]
            fn right() {
                let mut editor = Editor::new(Buffer::new("hello\nworld"));
                editor.handle_keypress(KeyCode::Char('l'));
                assert_eq!(editor.cursor().col(), 1);
            }

            #[test]
            fn jump_to_eol() {
                let mut editor = Editor::new(Buffer::new("012345"));
                editor.handle_keypress(KeyCode::Char('$'));
                assert_eq!(editor.cursor().col(), 5);
            }

            #[test]
            fn jump_to_start_of_line() {
                let mut editor = Editor::new(Buffer::new("012345"));

                editor.handle_keypress(KeyCode::Char('$'));
                editor.handle_keypress(KeyCode::Char('0'));

                assert_eq!(editor.cursor().col(), 0);
            }
        }

        mod commands {
            use super::*;

            #[test]
            fn quit() {
                let mut editor = Editor::new(Buffer::new(""));

                let action = editor.handle_keypress(KeyCode::Char('q'));

                assert_eq!(action, Some(EditorAction::Quit));
            }

            #[test]
            fn insert() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(KeyCode::Char('i'));
                assert_eq!(editor.mode(), &EditorMode::Insert);
            }
        }
    }

    mod insert_mode {
        use super::*;

        mod commands {
            use super::*;

            #[test]
            fn esc() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(KeyCode::Char('i'));
                editor.handle_keypress(KeyCode::Esc);

                assert_eq!(editor.mode(), &EditorMode::Normal);
            }
        }

        mod editing {
            use super::*;

            #[test]
            fn chars() {
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

            mod backspace {
                use super::*;

                #[test]
                fn removes_character() {
                    let mut editor = Editor::new(Buffer::new("Seppuku"));

                    // move right
                    editor.handle_keypress(KeyCode::Char('l'));

                    // switch to insert mode
                    editor.handle_keypress(KeyCode::Char('i'));

                    editor.handle_keypress(KeyCode::Backspace);

                    assert_eq!(editor.buffer().to_string(), "eppuku");
                }

                #[test]
                fn moves_cursor_back() {
                    let mut editor = Editor::new(Buffer::new("Seppuku"));

                    // move right
                    editor.handle_keypress(KeyCode::Char('l'));

                    // switch to insert mode
                    editor.handle_keypress(KeyCode::Char('i'));

                    editor.handle_keypress(KeyCode::Backspace);

                    assert_eq!(editor.cursor().col(), 0);
                }

                #[test]
                fn at_start_of_file() {
                    let mut editor = Editor::new(Buffer::new("Hello\nWorld"));

                    // switch to insert mode
                    editor.handle_keypress(KeyCode::Char('i'));

                    editor.handle_keypress(KeyCode::Backspace);
                    assert_eq!(editor.buffer().to_string(), "Hello\nWorld");
                    assert_eq!(editor.cursor().row(), 0);
                    assert_eq!(editor.cursor().col(), 0);
                }

                #[test]
                fn at_start_of_line() {
                    let mut editor = Editor::new(Buffer::new("01234\nWorld"));

                    // move down
                    editor.handle_keypress(KeyCode::Char('j'));

                    // switch to insert mode
                    editor.handle_keypress(KeyCode::Char('i'));

                    editor.handle_keypress(KeyCode::Backspace);

                    assert_eq!(editor.buffer().to_string(), "01234World");
                    assert_eq!(editor.cursor().row(), 0);
                    assert_eq!(editor.cursor().col(), 5);
                }
            }

            #[test]
            fn enter() {
                let mut editor = Editor::new(Buffer::new("Hello World"));

                // move right 5 times
                for _ in 0..5 {
                    editor.handle_keypress(KeyCode::Char('l'));
                }

                // switch to insert mode
                editor.handle_keypress(KeyCode::Char('i'));

                // insert a newline character
                editor.handle_keypress(KeyCode::Enter);

                assert_eq!(editor.buffer().to_string(), "Hello\n World");
                assert_eq!(editor.cursor().row(), 1);
                assert_eq!(editor.cursor().col(), 0);
            }
        }
    }
}
