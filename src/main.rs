use std::fs;
use std::io::stdout;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, style, terminal,
};

use rim::Cursor;

fn main() {
    let buffer = fs::read_to_string("foo.txt").unwrap();
    let mut terminal = Terminal::new();
    terminal.render(&buffer);
    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                if key_event.code == KeyCode::Char('j') {
                    terminal.move_cursor_down();
                }
                if key_event.code == KeyCode::Char('k') {
                    terminal.move_cursor_up();
                }
                if key_event.code == KeyCode::Char('l') {
                    terminal.move_cursor_right();
                }
                if key_event.code == KeyCode::Char('h') {
                    terminal.move_cursor_left();
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

    pub fn move_cursor_down(&mut self) {
        self.cursor.down();
        self.render_cursor();
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor.up();
        self.render_cursor();
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor.right();
        self.render_cursor();
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor.left();
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
        execute!(
            stdout(),
            cursor::MoveTo(self.cursor.col(), self.cursor.row())
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
