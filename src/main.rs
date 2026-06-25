use std::fs;

use crossterm::event::{read, Event, KeyCode};

use rim::{Buffer, Editor, Terminal};

fn main() {
    let contents = fs::read_to_string("foo.txt").unwrap();
    let buffer = Buffer::new(&contents);
    let terminal = Terminal::new();
    let editor = Editor::new(buffer);

    run(terminal, editor);
}

fn run(terminal: Terminal, mut editor: Editor) {
    terminal.render(editor.buffer(), editor.cursor());

    loop {
        match read().unwrap() {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('j') => editor.move_cursor_down(),
                KeyCode::Char('k') => editor.move_cursor_up(),
                KeyCode::Char('l') => editor.move_cursor_right(),
                KeyCode::Char('h') => editor.move_cursor_left(),
                key => {
                    editor.handle_keypress(key);
                }
            },
            _ => {}
        }

        terminal.render(editor.buffer(), editor.cursor());
    }
}
