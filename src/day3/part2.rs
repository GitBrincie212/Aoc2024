use num_bigint::BigInt;
use regex::{Match, Regex};
use crate::day3::capture_valid_instructions;
use crate::utils::read_input;

pub fn compute() {
    let content: String = read_input(3);
    let groups: Vec<(BigInt, BigInt, BigInt)> = capture_valid_instructions(&content);
    let mut sum: BigInt = BigInt::ZERO;
    let do_dont_pattern: Regex = Regex::new(r"(do|don't)\(\)").unwrap();

    /*
    This solution can be optimized by only computing the new previous do() and don't() and keeping
    a boolean value that indicates so far if its enabled. However, this should do the trick
     */
    for (num1, num2, index) in groups {
        let mut enabled: bool = true;
        for cap in do_dont_pattern.captures_iter(&content) {
            let cap_match: Match = cap.get(0).unwrap();
            if BigInt::from(cap_match.start()) < index {
                enabled = match cap_match.as_str() {
                    "do()" => {true}
                    _ => {false}
                };
            }
        }
        if !enabled {
            continue
        }
        sum += num1 * num2;
    }

    println!("{}", sum);
}