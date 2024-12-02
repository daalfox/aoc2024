use std::{env, fs};

use c2::{is_safe, parse_input};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let reports: Vec<Vec<usize>> = parse_input(input);

    let result = reports.into_iter().fold(0, |acc, report| {
        if is_safe(&report) {
            acc + 1
        } else {
            let mut is_safe_with_pd = false;
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if is_safe(&r) {
                    is_safe_with_pd = true;
                    break;
                }
            }

            if is_safe_with_pd {
                acc + 1
            } else {
                acc
            }
        }
    });

    println!("{}", result);
}
