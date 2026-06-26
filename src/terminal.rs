use std::io::stdout;

use crate::{editor::EditorMode, Buffer, Cursor};

use crossterm::{cursor, cursor::SetCursorStyle, execute, style, terminal};

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal
    }

    pub fn render(&self, buffer: &Buffer, cursor: &Cursor, mode: &EditorMode) {
        self.clear_screen();

        self.set_cursor_style(mode);

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

    fn set_cursor_style(&self, mode: &EditorMode) {
        match mode {
            EditorMode::Normal => execute!(stdout(), SetCursorStyle::SteadyBlock).unwrap(),
            EditorMode::Insert => execute!(stdout(), SetCursorStyle::SteadyBar).unwrap(),
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.clear_screen();
        terminal::disable_raw_mode().unwrap();
    }
}
