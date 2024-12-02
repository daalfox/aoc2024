use std::{env, fs};

use c2::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let reports: Vec<Vec<usize>> = parse_input(input);

    let result = reports.into_iter().fold(0, |acc, report| {
        if !report.is_sorted_by(|a, b| a < b) && !report.is_sorted_by(|a, b| a > b) {
            return acc;
        }

        if report.is_sorted_by(|a, b| a.abs_diff(*b) <= 3) {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", result);
}
