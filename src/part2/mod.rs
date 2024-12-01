use std::collections::HashSet;
use num_bigint::BigInt;
use crate::utils;

pub fn compute() {
    let contents: String = utils::read_input(2);
    let (left, right) = utils::split_in_two(&contents);
    let left_hashset: HashSet<BigInt> = HashSet::from_iter(left.into_iter());
    let mut sum: BigInt = BigInt::ZERO;
    for i in left_hashset {
        let count: BigInt = BigInt::from(right.iter().filter(|&x| *x == i ).count());
        sum += count * i;
    }
    println!("{}", sum);
}