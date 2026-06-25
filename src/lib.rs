pub mod editor;
pub mod terminal;

pub use editor::Editor;
pub use terminal::Terminal;

pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    pub fn new(content: &str) -> Self {
        let lines = content.lines().map(|line| line.to_string()).collect();

        Buffer { lines }
    }

    pub fn max_row(&self) -> usize {
        self.lines.len() - 1
    }

    pub fn max_col(&self, row: usize) -> usize {
        if self.lines.is_empty() {
            return 0;
        }

        let len = self.lines[row].len();

        if len == 0 {
            0
        } else {
            len - 1
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}

#[derive(Default)]
pub struct Cursor {
    row: u16,
    col: u16,
}

impl Cursor {
    pub fn new(row: u16, col: u16) -> Self {
        Cursor { row, col }
    }

    pub fn down(&mut self, buffer: &Buffer) {
        if self.row as usize >= buffer.max_row() {
            return;
        }

        self.row += 1;

        let next_line_max_col = buffer.max_col(self.row as usize);

        if self.col as usize >= next_line_max_col {
            self.col = next_line_max_col as u16;
        }
    }

    pub fn up(&mut self, buffer: &Buffer) {
        if self.row == 0 {
            return;
        }

        self.row -= 1;

        let next_line_max_col = buffer.max_col(self.row as usize);

        if self.col as usize >= next_line_max_col {
            self.col = next_line_max_col as u16;
        }
    }

    pub fn left(&mut self) {
        if self.col == 0 {
            return;
        }

        self.col -= 1;
    }

    pub fn right(&mut self, buffer: &Buffer) {
        if self.col as usize >= buffer.max_col(self.row as usize) {
            return;
        }
        self.col += 1;
    }

    pub fn row(&self) -> u16 {
        self.row
    }

    pub fn col(&self) -> u16 {
        self.col
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_buffer_max_row() {
        let contents = "\
zeroth line
oneth line
threethline";
        let buffer = Buffer::new(contents);

        assert_eq!(buffer.max_row(), 2);
    }

    #[test]
    fn test_buffer_max_col() {
        let contents = "\
this line has 24 columns
oneth line
threethline";

        let buffer = Buffer::new(contents);

        assert_eq!(buffer.max_col(0), 23);
    }

    #[test]
    fn test_buffer_max_col_of_empty_line() {
        let buffer = Buffer::new("");
        assert_eq!(buffer.max_col(0), 0);
    }
}

#[cfg(test)]
mod cursor_tests {
    use super::*;

    #[test]
    fn test_cursor_left_goes_left() {
        let mut cursor = Cursor::new(1, 10);

        cursor.left();

        assert_eq!([cursor.row(), cursor.col()], [1, 9]);
    }

    #[test]
    fn test_cursor_cannot_go_left_from_first_column() {
        let mut cursor = Cursor::new(1, 0);

        cursor.left();

        assert_eq!(cursor.row(), 1);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_cursor_goes_up() {
        let buffer = Buffer::new("000\n111\n2");
        let mut cursor = Cursor::new(1, 2);

        cursor.up(&buffer);

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn test_cursor_cannot_go_up_from_first_row() {
        let buffer = Buffer::new("000\n111\n2");
        let mut cursor = Cursor::new(0, 2);

        cursor.up(&buffer);

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 2);
    }

    #[test]
    fn test_cursor_goes_up_to_last_col_of_shorter_line() {
        let contents = "\
goes here ^
cursor here ^";

        let buffer = Buffer::new(contents);
        let mut cursor = Cursor::new(1, 12);

        cursor.up(&buffer);

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_goes_down() {
        let buffer = Buffer::new("0\n1\n2");
        let mut cursor = Cursor::new(1, 0);

        cursor.down(&buffer);

        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_cursor_cannot_go_down_from_last_row() {
        let buffer = Buffer::new("0\n1\n2");

        let mut cursor = Cursor::new(2, 0);

        cursor.down(&buffer);

        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 0);
    }

    #[test]
    fn test_cursor_goes_down_to_last_col_of_shorter_line() {
        let contents = "\
cursor here ^
goes here ^";

        let buffer = Buffer::new(contents);
        let mut cursor = Cursor::new(0, 12);

        cursor.down(&buffer);

        assert_eq!(cursor.row(), 1);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_goes_right() {
        let buffer = Buffer::new("0\n11\n2");
        let mut cursor = Cursor::new(1, 0);

        cursor.right(&buffer);

        assert_eq!(cursor.row(), 1);
        assert_eq!(cursor.col(), 1);
    }

    #[test]
    fn test_cursor_cannot_go_right_from_end_of_line() {
        let buffer = Buffer::new("0\n1\n2");
        let mut cursor = Cursor::new(0, 0);

        cursor.right(&buffer);

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 0);
    }
}
