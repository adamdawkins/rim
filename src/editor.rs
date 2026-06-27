use std::fmt;

use crate::{Buffer, Cursor};

const TAB_WIDTH: usize = 2;

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    mode: EditorMode,
    pending_command: Option<String>,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Self {
        Editor {
            buffer,
            cursor: Cursor::default(),
            mode: EditorMode::Normal,
            pending_command: None,
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

    pub fn written(&mut self) {
        self.buffer.mark_clean();
    }

    pub fn handle_keypress(&mut self, key: Key) -> Option<EditorAction> {
        match self.mode {
            EditorMode::Normal => self.handle_normal_mode_keypress(key),
            EditorMode::Insert => self.handle_insert_mode_keypress(key),
            EditorMode::Command => self.handle_command_mode_keypress(key),
        }
    }

    fn handle_normal_mode_keypress(&mut self, key: Key) -> Option<EditorAction> {
        match key {
            // Commands
            Key::Char('w') => Some(EditorAction::Write),
            Key::Char('q') => Some(EditorAction::Quit),
            Key::Char('i') => {
                self.mode = EditorMode::Insert;
                None
            }
            Key::Char(':') => {
                self.mode = EditorMode::Command;
                None
            }

            // Motions
            Key::Char('h') => {
                self.move_cursor_left();
                None
            }
            Key::Char('j') => {
                self.move_cursor_down();
                None
            }
            Key::Char('k') => {
                self.move_cursor_up();
                None
            }
            Key::Char('l') => {
                self.move_cursor_right();
                None
            }
            Key::Char('0') => {
                self.move_cursor_to_start_of_line(self.cursor.row());
                None
            }
            Key::Char('^') => {
                self.move_cursor_to_first_non_whitespace_char(self.cursor.row());
                None
            }
            Key::Char('$') => {
                self.move_cursor_to_end_of_line(self.cursor.row());
                None
            }
            _ => None,
        }
    }

    fn handle_command_mode_keypress(&mut self, key: Key) -> Option<EditorAction> {
        match key {
            Key::Esc => {
                self.mode = EditorMode::Normal;
                None
            }
            _ => None,
        }
    }

    fn handle_insert_mode_keypress(&mut self, key: Key) -> Option<EditorAction> {
        match key {
            Key::Esc => {
                self.mode = EditorMode::Normal;
                None
            }
            Key::Backspace => {
                self.backspace();
                None
            }
            Key::Enter => {
                self.enter();
                None
            }
            Key::Tab => {
                self.tab();
                None
            }
            Key::Char(c) => {
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

    fn move_cursor_to_first_non_whitespace_char(&mut self, row: usize) {
        let col = self.buffer.first_non_whitespace_col(row);
        self.cursor.move_to(row, col);
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

    fn tab(&mut self) {
        for _ in 0..TAB_WIDTH {
            self.insert_char(' ');
        }
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
    Command,
}

impl fmt::Display for EditorMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EditorMode::Normal => write!(f, "Normal"),
            EditorMode::Insert => write!(f, "Insert"),
            EditorMode::Command => write!(f, "Command"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EditorAction {
    Quit,
    Write,
}

pub enum Key {
    Char(char),
    Esc,
    Enter,
    Backspace,
    Tab,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod normal_mode {
        use super::*;

        mod switches {
            use super::*;

            #[test]
            fn insert() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(Key::Char('i'));
                assert_eq!(editor.mode(), &EditorMode::Insert);
            }

            #[test]
            fn command() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(Key::Char(':'));
                assert_eq!(editor.mode(), &EditorMode::Command);
            }
        }

        mod actions {
            use super::*;

            #[test]
            fn quit() {
                let mut editor = Editor::new(Buffer::new(""));

                let action = editor.handle_keypress(Key::Char('q'));

                assert_eq!(action, Some(EditorAction::Quit));
            }

            #[test]
            fn write() {
                let mut editor = Editor::new(Buffer::new(""));

                let action = editor.handle_keypress(Key::Char('w'));

                assert_eq!(action, Some(EditorAction::Write));
            }
        }

        mod motions {
            use super::*;

            #[test]
            fn left() {
                let mut editor = Editor::new(Buffer::new("hello\nworld"));
                editor.handle_keypress(Key::Char('l'));
                editor.handle_keypress(Key::Char('h'));
                assert_eq!(editor.cursor().col(), 0);
            }

            #[test]
            fn down() {
                let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
                editor.handle_keypress(Key::Char('j'));
                assert_eq!(editor.cursor().row(), 1);
            }

            #[test]
            fn up() {
                let mut editor = Editor::new(Buffer::new("line0\nline1\nline2"));
                editor.handle_keypress(Key::Char('j'));
                editor.handle_keypress(Key::Char('k'));
                assert_eq!(editor.cursor().row(), 0);
            }

            #[test]
            fn right() {
                let mut editor = Editor::new(Buffer::new("hello\nworld"));
                editor.handle_keypress(Key::Char('l'));
                assert_eq!(editor.cursor().col(), 1);
            }

            #[test]
            fn jump_to_eol() {
                let mut editor = Editor::new(Buffer::new("012345"));
                editor.handle_keypress(Key::Char('$'));
                assert_eq!(editor.cursor().col(), 5);
            }

            #[test]
            fn jump_to_start_of_line() {
                let mut editor = Editor::new(Buffer::new("012345"));

                editor.handle_keypress(Key::Char('$'));
                editor.handle_keypress(Key::Char('0'));

                assert_eq!(editor.cursor().col(), 0);
            }

            #[test]
            fn jump_to_first_non_blank_char() {
                let mut editor = Editor::new(Buffer::new("    012345"));

                editor.handle_keypress(Key::Char('^'));

                assert_eq!(editor.cursor().col(), 4);
            }
        }
    }

    mod command_mode {
        use super::*;

        mod switches {
            use super::*;

            #[test]
            fn esc() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(Key::Char(':'));
                editor.handle_keypress(Key::Esc);

                assert_eq!(editor.mode(), &EditorMode::Normal);
            }
        }
    }

    mod insert_mode {
        use super::*;

        mod io {
            use super::*;

            #[test]
            fn written() {
                let mut editor = Editor::new(Buffer::new("Hello"));

                editor.handle_keypress(Key::Char('i'));
                editor.handle_keypress(Key::Char('h'));
                assert!(editor.buffer().is_modified());

                editor.written();
                assert!(!editor.buffer().is_modified());
            }
        }

        mod commands {
            use super::*;

            #[test]
            fn esc() {
                let mut editor = Editor::new(Buffer::new(""));

                editor.handle_keypress(Key::Char('i'));
                editor.handle_keypress(Key::Esc);

                assert_eq!(editor.mode(), &EditorMode::Normal);
            }
        }

        mod editing {
            use super::*;

            #[test]
            fn chars() {
                let mut editor = Editor::new(Buffer::new("Hello"));

                // switch to insert mode
                editor.handle_keypress(Key::Char('i'));

                editor.handle_keypress(Key::Char('h'));
                editor.handle_keypress(Key::Char('j'));
                editor.handle_keypress(Key::Char('k'));
                editor.handle_keypress(Key::Char('l'));
                editor.handle_keypress(Key::Char('i'));
                editor.handle_keypress(Key::Char('q'));

                assert_eq!(editor.buffer().to_string(), "hjkliqHello");
            }

            mod backspace {
                use super::*;

                #[test]
                fn removes_character() {
                    let mut editor = Editor::new(Buffer::new("Seppuku"));

                    // move right
                    editor.handle_keypress(Key::Char('l'));

                    // switch to insert mode
                    editor.handle_keypress(Key::Char('i'));

                    editor.handle_keypress(Key::Backspace);

                    assert_eq!(editor.buffer().to_string(), "eppuku");
                }

                #[test]
                fn moves_cursor_back() {
                    let mut editor = Editor::new(Buffer::new("Seppuku"));

                    // move right
                    editor.handle_keypress(Key::Char('l'));

                    // switch to insert mode
                    editor.handle_keypress(Key::Char('i'));

                    editor.handle_keypress(Key::Backspace);

                    assert_eq!(editor.cursor().col(), 0);
                }

                #[test]
                fn at_start_of_file() {
                    let mut editor = Editor::new(Buffer::new("Hello\nWorld"));

                    // switch to insert mode
                    editor.handle_keypress(Key::Char('i'));

                    editor.handle_keypress(Key::Backspace);
                    assert_eq!(editor.buffer().to_string(), "Hello\nWorld");
                    assert_eq!(editor.cursor().row(), 0);
                    assert_eq!(editor.cursor().col(), 0);
                }

                #[test]
                fn at_start_of_line() {
                    let mut editor = Editor::new(Buffer::new("01234\nWorld"));

                    // move down
                    editor.handle_keypress(Key::Char('j'));

                    // switch to insert mode
                    editor.handle_keypress(Key::Char('i'));

                    editor.handle_keypress(Key::Backspace);

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
                    editor.handle_keypress(Key::Char('l'));
                }

                // switch to insert mode
                editor.handle_keypress(Key::Char('i'));

                // insert a newline character
                editor.handle_keypress(Key::Enter);

                assert_eq!(editor.buffer().to_string(), "Hello\n World");
                assert_eq!(editor.cursor().row(), 1);
                assert_eq!(editor.cursor().col(), 0);
            }

            #[test]
            fn tab() {
                let mut editor = Editor::new(Buffer::new("Hello World"));

                editor.handle_keypress(Key::Char('l'));
                editor.handle_keypress(Key::Char('i'));
                editor.handle_keypress(Key::Tab);

                // TAB_WIDTH currently is 2, so we expect two spaces to be inserted
                assert_eq!(editor.buffer().to_string(), "H  ello World");
                assert_eq!(editor.cursor().row(), 0);
                assert_eq!(editor.cursor().col(), 3);
            }
        }
    }
}
