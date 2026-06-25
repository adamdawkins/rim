use std::fs;
use std::io::stdout;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute, style, terminal,
};

use rim::{editor::Editor, Buffer, Cursor};

fn main() {
    let contents = fs::read_to_string("foo.txt").unwrap();
    let buffer = Buffer::new(&contents);
    let mut terminal = Terminal::new();
    let editor = Editor::new(buffer);

    terminal.render(&editor.buffer());

    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                if key_event.code == KeyCode::Char('j') {
                    terminal.move_cursor_down(&editor.buffer());
                }
                if key_event.code == KeyCode::Char('k') {
                    terminal.move_cursor_up(&editor.buffer());
                }
                if key_event.code == KeyCode::Char('l') {
                    terminal.move_cursor_right(&editor.buffer());
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

    pub fn render(&self, buffer: &Buffer) {
        self.clear_screen();

        for line in buffer.lines() {
            execute!(stdout(), style::Print(format!("{}\r\n", line))).unwrap();
        }

        self.render_cursor();
    }

    pub fn move_cursor_down(&mut self, buffer: &Buffer) {
        self.cursor.down(&buffer);
        self.render_cursor();
    }

    pub fn move_cursor_up(&mut self, buffer: &Buffer) {
        self.cursor.up(&buffer);
        self.render_cursor();
    }

    pub fn move_cursor_right(&mut self, buffer: &Buffer) {
        self.cursor.right(&buffer);
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
