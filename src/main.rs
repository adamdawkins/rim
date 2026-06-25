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
                    terminal.render(&editor.buffer(), &editor.cursor());
                }
                if key_event.code == KeyCode::Char('k') {
                    editor.move_cursor_up();
                    terminal.render(&editor.buffer(), &editor.cursor());
                }
                if key_event.code == KeyCode::Char('l') {
                    editor.move_cursor_right();
                    terminal.render(&editor.buffer(), &editor.cursor());
                }
                if key_event.code == KeyCode::Char('h') {
                    editor.move_cursor_left();
                    terminal.render(&editor.buffer(), &editor.cursor());
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

    pub fn render(&self, buffer: &Buffer, cursor: &Cursor) {
        self.clear_screen();

        for line in buffer.lines() {
            execute!(stdout(), style::Print(format!("{}\r\n", line))).unwrap();
        }

        Terminal::render_cursor(cursor);
    }

    pub fn move_cursor_down(&mut self, buffer: &Buffer) {
        self.cursor.down(&buffer);
        Terminal::render_cursor(&self.cursor);
    }

    pub fn move_cursor_up(&mut self, buffer: &Buffer) {
        self.cursor.up(&buffer);
        Terminal::render_cursor(&self.cursor);
    }

    pub fn move_cursor_right(&mut self, buffer: &Buffer) {
        self.cursor.right(&buffer);
        Terminal::render_cursor(&self.cursor);
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor.left();
        Terminal::render_cursor(&self.cursor);
    }

    fn clear_screen(&self) {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
    }

    fn render_cursor(cursor: &Cursor) {
        execute!(stdout(), cursor::MoveTo(cursor.col(), cursor.row())).unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
