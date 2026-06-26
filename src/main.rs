use std::fs;

use crossterm::event::{read, Event};

use rim::{editor::EditorAction, Buffer, Editor, Terminal};

fn main() {
    let contents = fs::read_to_string("foo.txt").unwrap();
    let buffer = Buffer::new(&contents);
    let terminal = Terminal::new();
    let editor = Editor::new(buffer);

    run(terminal, editor);
}

fn run(terminal: Terminal, mut editor: Editor) {
    terminal.render(editor.buffer(), editor.cursor(), editor.mode());

    loop {
        match read().unwrap() {
            Event::Key(key_event) => {
                if let Some(EditorAction::Quit) = editor.handle_keypress(key_event.code) {
                    break;
                }
            }
            _ => {}
        }

        terminal.render(editor.buffer(), editor.cursor(), editor.mode());
    }
}
