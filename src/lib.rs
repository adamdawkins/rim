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
mod tests {
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

        assert_eq!([cursor.row(), cursor.col()], [1, 0]);
    }
}
