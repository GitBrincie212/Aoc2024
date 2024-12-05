use num_bigint::BigInt;
use crate::day2::{run_analysis_and_conclude};
use crate::utils::read_input;

pub fn compute() {
    let content: String = read_input(2);
    let mut safe_reports: BigInt = BigInt::ZERO;
    content.lines().for_each(|line| { parse_data(line, &mut safe_reports) });
    println!("{}", safe_reports);
}

pub fn parse_data(line: &str, safe_reports: &mut BigInt) {
    let data: Vec<&str> = line.split(" ").collect();
    /*
    Solved it via brute-forcing. Very inefficient as it leads to ~O(n^2), not very happy about it,
    but at least it works. I might retouch on this problem and think of a more clever algorithm
    */
    for i in 0..data.len() {
        let mut copied_data: Vec<&str> = data.clone();
        copied_data.remove(i);
        let analysis: (bool, usize) = run_analysis_and_conclude(&copied_data, safe_reports);
        if analysis.0 {
            return;
        }
    }
}