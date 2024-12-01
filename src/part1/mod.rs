use num_bigint::{BigInt, Sign};
use crate::utils;

pub fn compute() {
    let contents: String = utils::read_input(1);
    let (mut left, mut right) = utils::split_in_two(&contents);

    left.sort();
    right.sort();

    let mut sum: BigInt = BigInt::ZERO;
    for i in 0..(contents.lines().count()) {
        let result: BigInt = &left[i] - &right[i];
        sum += BigInt::new(Sign::Plus, result.to_u32_digits().1)
    }

    println!("{}", sum);
}