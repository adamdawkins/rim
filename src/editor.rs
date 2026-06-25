use crate::{Buffer, Cursor};

pub struct Editor {
    buffer: Buffer,
    cursor: Cursor,
    mode: EditorMode,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Self {
        Editor {
            buffer,
            cursor: Cursor::default(),
            mode: EditorMode::Normal,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor.down(&self.buffer);
    }
}

enum EditorMode {
    Normal,
    // Insert,
}
