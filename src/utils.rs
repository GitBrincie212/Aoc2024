use std::fs;
use num_bigint::BigInt;
use std::str::{FromStr, Lines};

pub fn read_input(part: u8) -> String {
    fs::read_to_string(format!("./src/part{}/input.txt", part))
        .expect("Should have been able to read the file")
}

pub fn split_in_two(input: &String) -> (Vec<BigInt>, Vec<BigInt>) {
    let lines: Lines = input.lines();

    let line_count: usize = lines.clone().count();

    let mut col_one: Vec<BigInt> = Vec::with_capacity(line_count);
    let mut col_two: Vec<BigInt> = Vec::with_capacity(line_count);

    for line in lines {
        let data: Vec<&str> = line.split("   ").collect();
        let left_num: BigInt = BigInt::from_str(data[0]).unwrap();
        let right_num: BigInt = BigInt::from_str(data[1]).unwrap();
        col_one.push(left_num);
        col_two.push(right_num);
    }

    (col_one, col_two)
}