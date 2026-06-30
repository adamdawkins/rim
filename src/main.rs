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
    editor::{EditorAction, EditorMode, Key, Message},
    Buffer, Cursor, Editor,
};

fn main() {
    let path = "foo.txt";
    let contents = fs::read_to_string(path).unwrap();
    let buffer = Buffer::with_path(&contents, Some(path.to_string()));
    let _terminal = Terminal::new();
    let editor = Editor::new(buffer);

    run(editor);
}

fn run(mut editor: Editor) {
    Terminal::render(
        editor.buffer(),
        editor.cursor(),
        editor.mode(),
        editor.pending_command(),
        editor.message(),
    );

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
                Some(EditorAction::WriteAndQuit) => {
                    if fs::write("foo.txt", editor.buffer().to_string()).is_ok() {
                        editor.written();
                    }
                    break;
                }
                None => {}
            },
            _ => {}
        }

        Terminal::render(
            editor.buffer(),
            editor.cursor(),
            editor.mode(),
            editor.pending_command(),
            editor.message(),
        );
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

    pub fn render(
        buffer: &Buffer,
        cursor: &Cursor,
        mode: &EditorMode,
        pending_command: Option<&str>,
        message: Option<&Message>,
    ) {
        Self::clear_screen();
        Self::set_cursor_style(mode);
        Self::render_buffer(buffer);
        Self::render_status_line(mode, buffer, cursor);
        Self::render_command_line(mode, pending_command, message);
        Self::render_cursor(mode, cursor, pending_command);
    }

    fn clear_screen() {
        execute!(
            stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
    }

    fn set_cursor_style(mode: &EditorMode) {
        match mode {
            EditorMode::Normal => execute!(stdout(), SetCursorStyle::SteadyBlock).unwrap(),
            EditorMode::Insert => execute!(stdout(), SetCursorStyle::SteadyBar).unwrap(),
            _ => execute!(stdout(), SetCursorStyle::SteadyBlock).unwrap(),
        }
    }

    // rendering
    fn render_buffer(buffer: &Buffer) {
        for line in buffer.lines() {
            execute!(stdout(), Print(format!("{}\r\n", line))).unwrap();
        }
    }

    fn render_cursor(mode: &EditorMode, buffer_cursor: &Cursor, pending_command: Option<&str>) {
        let mut row = buffer_cursor.row() as u16;
        let mut col = buffer_cursor.col() as u16;

        if mode == &EditorMode::Command {
            row = terminal::size().unwrap().1;
            col = pending_command.map_or(1, |cmd| cmd.len() as u16 + 1)
        }

        execute!(stdout(), cursor::MoveTo(col as u16, row as u16)).unwrap();
    }

    fn render_status_line(mode: &EditorMode, buffer: &Buffer, cursor: &Cursor) {
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
            cursor::MoveTo(0, terminal::size().unwrap().1 - 2),
            Print(status),
            ResetColor
        )
        .unwrap();
    }

    fn render_command_line(
        mode: &EditorMode,
        pending_command: Option<&str>,
        message: Option<&Message>,
    ) {
        let line = format!(
            "{}",
            if mode == &EditorMode::Command {
                format!(":{}", pending_command.unwrap_or(""))
            } else {
                match message {
                    Some(Message::Info(msg)) => format!("{}", msg),
                    Some(Message::Error(msg)) => format!("Err: {}", msg),
                    None => String::new(),
                }
            },
        );

        if matches!(message, Some(Message::Error(_))) {
            execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
        }

        execute!(
            stdout(),
            cursor::MoveTo(0, terminal::size().unwrap().1 - 1),
            Print(line),
            ResetColor
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        Terminal::clear_screen();
        terminal::disable_raw_mode().unwrap();
    }
}
