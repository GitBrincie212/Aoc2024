use std::ops::AddAssign;
use std::str::FromStr;
use num_bigint::BigInt;
use num_traits::Signed;

pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq, Clone)]
pub enum GradualType {
    UNSET,
    INCREASING,
    DECREASING
}

pub fn set_gradual<'a> (diff: &BigInt) -> Option<&'a GradualType> {
    if diff > &BigInt::ZERO {
        return Some(&GradualType::INCREASING);
    } else if diff < &BigInt::ZERO {
        return Some(&GradualType::DECREASING);
    }
    None
}

pub fn gradual_to_bool(diff: &BigInt, gradual_type: &GradualType) -> bool {
    (diff > &BigInt::ZERO && gradual_type == &GradualType::INCREASING) ||
            (diff < &BigInt::ZERO && gradual_type == &GradualType::DECREASING)
}

pub fn analyse_single_report<'a>(data: &Vec<&str>, mut gradual_type: &GradualType) -> (bool, usize) {
    if data.len() >= 1 {
        let mut prev_entry: BigInt = BigInt::from_str(data[0]).unwrap();
        let mut index: usize = 0;
        for str_num in &data[1..] {
            let num: BigInt = BigInt::from_str(*str_num).unwrap();
            let signed_diff: BigInt = &prev_entry - &num;
            let diff: BigInt = signed_diff.abs();
            let diff_in_range: bool = (BigInt::from(1)..=BigInt::from(3))
                .contains(&diff);
            let mut option_gradual = Some(gradual_type);
            if gradual_type == &GradualType::UNSET {
                option_gradual = set_gradual(&signed_diff);
            }
            if option_gradual.is_none() {
                return (false, index);
            }
            gradual_type = option_gradual.unwrap();
            let is_gradual: bool = gradual_to_bool(&signed_diff, gradual_type);
            if !(diff_in_range && is_gradual) {
                return (false, index);
            }
            prev_entry = num.clone();
            index.add_assign(1);
        }
    }
    (true, 0)
}

pub fn run_analysis_and_conclude(data: &Vec<&str>, safe_reports: &mut BigInt) -> (bool, usize) {
    let gradual_type: GradualType = GradualType::UNSET;
    let analysis: (bool, usize) = analyse_single_report(&data, &gradual_type);
    if analysis.0 {
        safe_reports.add_assign(1);
    }
    analysis
}