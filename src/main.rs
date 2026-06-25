use std::fs;

fn main() {
    let buffer = fs::read_to_string("foo.txt").unwrap();
    println!("{}", buffer);
}
