use std::io::stdout;

use crate::{Buffer, Cursor};

use crossterm::{cursor, execute, style, terminal};

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal
    }

    pub fn render(&self, buffer: &Buffer, cursor: &Cursor) {
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
        terminal::disable_raw_mode().unwrap();
    }
}
