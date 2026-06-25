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
    let terminal = Terminal::new();
    let mut editor = Editor::new(buffer);

    terminal.render(&editor.buffer(), &editor.cursor());

    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if key_event.code == KeyCode::Char('q') {
                    break;
                }
                if key_event.code == KeyCode::Char('j') {
                    editor.move_cursor_down();
                }
                if key_event.code == KeyCode::Char('k') {
                    editor.move_cursor_up();
                }
                if key_event.code == KeyCode::Char('l') {
                    editor.move_cursor_right();
                }
                if key_event.code == KeyCode::Char('h') {
                    editor.move_cursor_left();
                }
            }
            _ => {}
        }

        terminal.render(&editor.buffer(), &editor.cursor());
    }
}

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal {}
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
