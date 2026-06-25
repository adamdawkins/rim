use std::fs;
use std::io::stdout;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, style, terminal,
};

fn main() {
    let buffer = fs::read_to_string("foo.txt").unwrap();
    let terminal = Terminal::new();
    terminal.render(&buffer);
    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
            }
            _ => {}
        }
    }
}

pub struct Terminal {
    cursor: Cursor,
}

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal {
            cursor: Cursor::default(),
        }
    }

    pub fn render(&self, buffer: &str) {
        self.clear_screen();

        for line in buffer.lines() {
            execute!(stdout(), style::Print(format!("{}\r\n", line))).unwrap();
        }

        self.render_cursor();
    }

    fn clear_screen(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
    }

    fn render_cursor(&self) {
        execute!(stdout(), cursor::MoveTo(self.cursor.col, self.cursor.row)).unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}

#[derive(Default)]
pub struct Cursor {
    row: u16,
    col: u16,
}
