use std::fs;

use crossterm::terminal;

fn main() {
    let buffer = fs::read_to_string("foo.txt").unwrap();
    let _terminal = Terminal::new();
    println!("{}", buffer);
}

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();
        Terminal
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
