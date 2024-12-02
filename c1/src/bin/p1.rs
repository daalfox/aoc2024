use std::env;
use std::fs;

use c1::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (mut left, mut right): (Vec<usize>, Vec<usize>) = parse_input(input);

    left.sort();
    right.sort();
    right.reverse();

    let result = left.into_iter().fold(0, |acc, x| {
        let y = right.pop().unwrap();
        acc + x.abs_diff(y)
    });

    println!("{}", result);
}
