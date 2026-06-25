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
