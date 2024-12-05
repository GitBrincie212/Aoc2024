use num_bigint::BigInt;
use crate::day3::capture_valid_instructions;
use crate::utils::read_input;

pub fn compute() {
    let content: String = read_input(3);
    let groups: Vec<(BigInt, BigInt, BigInt)> = capture_valid_instructions(&content);
    let mut sum: BigInt = BigInt::ZERO;

    for (num1, num2, _) in groups {
        sum += num1 * num2;
    }

    println!("{}", sum);
}