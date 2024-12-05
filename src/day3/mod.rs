use std::str::FromStr;
use num_bigint::BigInt;
use regex::{Captures, Regex};

pub mod part1;
pub mod part2;

fn capture_valid_instructions(input: &str) -> Vec<(BigInt, BigInt, BigInt)> {
    let regex_pattern: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let vec: Vec<Captures> = regex_pattern.captures_iter(&input).collect();

    let mut groups: Vec<(BigInt, BigInt, BigInt)> = Vec::with_capacity(vec.len());
    for cap in vec {
        let index: BigInt = BigInt::from(cap.get(0).unwrap().start());
        let num1_str: &str = &cap[1];
        let num2_str: &str = &cap[2];

        let num1: BigInt = BigInt::from_str(num1_str).unwrap();
        let num2: BigInt = BigInt::from_str(num2_str).unwrap();

        groups.push((num1, num2, index));
    }
    groups
}