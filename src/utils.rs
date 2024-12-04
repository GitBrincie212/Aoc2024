use std::fs;

pub fn read_input(day: u8, part: u8) -> String {
    fs::read_to_string(format!("./src/day{}/part{}/input.txt", day, part))
        .expect("Should have been able to read the file")
}