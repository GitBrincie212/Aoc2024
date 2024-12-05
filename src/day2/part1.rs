use crate::day2::run_analysis_and_conclude;
use crate::utils::read_input;
use num_bigint::BigInt;

pub fn compute() {
    let content: String = read_input(2);
    let mut safe_reports: BigInt = BigInt::ZERO;
    content.lines().for_each(|line| {
        let data: Vec<&str> = line.split(" ").collect();
        run_analysis_and_conclude(&data, &mut safe_reports);
    });
    println!("{}", safe_reports);
}