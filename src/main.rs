use std::fs;
use std::io::stdout;

use crossterm::{
    cursor,
    cursor::SetCursorStyle,
    event::{read, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal,
};

use rim::{
    editor::{EditorAction, EditorMode, Key},
    Buffer, Cursor, Editor,
};

fn main() {
    let path = "foo.txt";
    let contents = fs::read_to_string(path).unwrap();
    let buffer = Buffer::with_path(&contents, Some(path.to_string()));
    let terminal = Terminal::new();
    let editor = Editor::new(buffer);

    run(terminal, editor);
}

fn run(terminal: Terminal, mut editor: Editor) {
    terminal.render(editor.buffer(), editor.cursor(), editor.mode());

    loop {
        match read().unwrap() {
            Event::Key(key_event) => match editor.handle_keypress(to_key(key_event.code)) {
                Some(EditorAction::Quit) => {
                    break;
                }
                Some(EditorAction::Write) => {
                    if fs::write("foo.txt", editor.buffer().to_string()).is_ok() {
                        editor.written();
                    }
                }
                _ => {}
            },
            _ => {}
        }

        terminal.render(editor.buffer(), editor.cursor(), editor.mode());
    }
}

fn to_key(code: KeyCode) -> Key {
    match code {
        KeyCode::Char(c) => Key::Char(c),
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Esc,
        KeyCode::Tab => Key::Tab,
        _ => Key::Other,
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
        self.render_buffer(buffer);
        self.render_status_line(mode, buffer, cursor);
        self.render_command_line(mode, buffer, cursor);
        self.render_cursor(mode, cursor);
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
            _ => execute!(stdout(), SetCursorStyle::SteadyBlock).unwrap(),
        }
    }

    // rendering
    fn render_buffer(&self, buffer: &Buffer) {
        for line in buffer.lines() {
            execute!(stdout(), Print(format!("{}\r\n", line))).unwrap();
        }
    }

    fn render_cursor(&self, _mode: &EditorMode, cursor: &Cursor) {
        execute!(
            stdout(),
            cursor::MoveTo(cursor.col() as u16, cursor.row() as u16)
        )
        .unwrap();
    }

    fn render_status_line(&self, mode: &EditorMode, buffer: &Buffer, cursor: &Cursor) {
        let path = format!(
            "{}{}",
            buffer.path().unwrap_or("[No Name]"),
            if buffer.is_modified() { " [+]" } else { "" }
        );

        let left = format!(" {} | {}", mode.to_string().to_uppercase(), path);
        let right = format!("{}:{} ", cursor.row() + 1, cursor.col() + 1);

        let width = terminal::size().unwrap().0 as usize;
        let gap = width.saturating_sub(left.len() + right.len());

        let status = format!("{}{:gap$}{}", left, "", right, gap = gap);

        execute!(
            stdout(),
            SetBackgroundColor(Color::Yellow),
            SetForegroundColor(Color::Black),
            cursor::MoveTo(0, terminal::size().unwrap().1 - 1),
            Print(status),
            ResetColor
        )
        .unwrap();
    }

    fn render_command_line(&self, mode: &EditorMode, _buffer: &Buffer, _cursor: &Cursor) {
        let line = format!(
            "{}",
            if mode == &EditorMode::Command {
                ":"
            } else {
                ""
            },
        );

        execute!(
            stdout(),
            cursor::MoveTo(0, terminal::size().unwrap().1),
            Print(line)
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.clear_screen();
        terminal::disable_raw_mode().unwrap();
    }
}
