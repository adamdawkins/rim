use std::io::stdout;

use crate::{editor::EditorMode, Buffer, Cursor};

use crossterm::{cursor, execute, style, terminal};

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal
    }

    pub fn render(&self, buffer: &Buffer, cursor: &Cursor, mode: &EditorMode) {
        self.clear_screen();

        for line in buffer.lines() {
            execute!(stdout(), style::Print(format!("{}\r\n", line))).unwrap();
        }

        execute!(stdout(), cursor::MoveTo(cursor.col(), cursor.row())).unwrap();
    }

    fn clear_screen(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.clear_screen();
        terminal::disable_raw_mode().unwrap();
    }
}
