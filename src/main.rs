use std::io::{stdin, stdout, Write};

pub mod utils;

mod day1;
mod day2;
mod day3;
mod day4;

fn read_as_str(message: &str) -> String {
    let mut day_str: String = String::from("");
    stdin()
        .read_line(&mut day_str)
        .expect(
            format!("Couldn't Read  \"{}\" Argument", message)
                .as_str()
        );
    day_str.trim().to_string()
}

fn main() {
    print!("\x1B[2J\x1B[1;1H"); // Clear the terminal screen
    print!("Input Day Number >> ");
    stdout().flush().unwrap();
    let binding: String = read_as_str("Day");
    let day: &str = binding.as_str();
    print!("Input Part Number >> ");
    stdout().flush().unwrap();
    let binding2: String = read_as_str("Part");
    let part: &str = binding2.as_str();

    match (day, part) {
        ("1", "1") => {day1::part1::compute()}
        ("1", "2") => {day1::part2::compute()}
        ("2", "1") => {day2::part1::compute()}
        ("2", "2") => {day2::part2::compute()}
        ("3", "1") => {day3::part1::compute()}
        ("3", "2") => {day3::part2::compute()}
        ("4", "1") => {day4::part1::compute()}
        _ => {println!("Invalid input. Couldn't find the combination")}
    }
}