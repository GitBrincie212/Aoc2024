use std::ops::AddAssign;
use std::str::{FromStr, Lines};
use num_bigint::BigInt;
use crate::utils::read_input;
use num_traits::sign::Signed;

#[derive(Debug, PartialEq, Clone)]
enum GradualType {
    UNSET,
    INCREASING,
    DECREASING
}

pub fn compute() {
    let content: String = read_input(2, 1);
    let gradual_type: GradualType = GradualType::UNSET;
    let mut safe_reports = BigInt::ZERO;
    content.lines().for_each(|line| { parse_data(line, &mut safe_reports, &gradual_type) });
    println!("{}", safe_reports);
}

pub fn parse_data(line: &str, safe_reports: &mut BigInt, mut gradual_type: &GradualType) {
    let data: Vec<&str> = line.split(" ").collect();
    if data.len() >= 1 {
        let mut prev_entry: BigInt = BigInt::from_str(data[0]).unwrap();
        for str_num in &data[1..] {
            let num: BigInt = BigInt::from_str(*str_num).unwrap();
            let signed_diff: BigInt = &prev_entry - &num;
            let diff: BigInt = signed_diff.abs();
            let diff_in_range: bool = (BigInt::from(1)..=BigInt::from(3))
                .contains(&diff);
            if gradual_type == &GradualType::UNSET {
                gradual_type = if signed_diff > BigInt::ZERO {
                    &GradualType::INCREASING
                } else if signed_diff < BigInt::ZERO {
                    &GradualType::DECREASING
                } else {
                    return;
                };
            }
            let is_gradual =
                (signed_diff > BigInt::ZERO && gradual_type == &GradualType::INCREASING) ||
                (signed_diff < BigInt::ZERO && gradual_type == &GradualType::DECREASING);
            if !(diff_in_range && is_gradual) {
                return;
            }
            prev_entry = num.clone();
        }
        println!("{}", line);
        safe_reports.add_assign(1);
    }
}