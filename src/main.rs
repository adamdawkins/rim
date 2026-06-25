use std::fs;

use crossterm::event::{read, Event, KeyCode};

use rim::{Buffer, Editor, Terminal};

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
