use std::fs;
use std::io::stdout;

use crossterm::{
    cursor,
    cursor::SetCursorStyle,
    event::{read, Event},
    execute, style, terminal,
};

use rim::{
    editor::{EditorAction, EditorMode},
    Buffer, Cursor, Editor,
};

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
            Event::Key(key_event) => match editor.handle_keypress(key_event.code) {
                Some(EditorAction::Quit) => {
                    break;
                }
                Some(EditorAction::Write) => {
                    fs::write("foo.txt", editor.buffer().to_string()).unwrap();
                }
                _ => {}
            },
            _ => {}
        }

        terminal.render(editor.buffer(), editor.cursor(), editor.mode());
    }
}

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

        execute!(
            stdout(),
            cursor::MoveTo(cursor.col() as u16, cursor.row() as u16)
        )
        .unwrap();
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
