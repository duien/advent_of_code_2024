const DAY: u8 = 0;
const TEST_INPUT: &str = "\
";

use std::fs;

fn main() {}

fn file_input() -> String {
    let file_path = format!("../ruby/data/day_{:02}.txt", DAY);
    fs::read_to_string(file_path).expect("unable to read file")
}
