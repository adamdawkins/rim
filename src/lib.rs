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
        self.lines[row].len() - 1
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

    pub fn down(&mut self) {
        self.row += 1;
    }

    pub fn up(&mut self) {
        if self.row == 0 {
            return;
        }

        self.row -= 1;
    }

    pub fn left(&mut self) {
        if self.col == 0 {
            return;
        }

        self.col -= 1;
    }

    pub fn right(&mut self) {
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
        let mut cursor = Cursor::new(1, 10);

        cursor.up();

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_cannot_go_up_from_first_row() {
        let mut cursor = Cursor::new(0, 10);

        cursor.up();

        assert_eq!(cursor.row(), 0);
        assert_eq!(cursor.col(), 10);
    }

    #[test]
    fn test_cursor_goes_down() {
        let mut cursor = Cursor::new(1, 10);

        cursor.down();

        assert_eq!(cursor.row(), 2);
        assert_eq!(cursor.col(), 10);
    }
}
