use std::fs;

pub fn read_input(day: u8) -> String {
    fs::read_to_string(format!("./src/day{}/input.txt", day))
        .expect("Should have been able to read the file")
}