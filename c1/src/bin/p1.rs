use std::env;
use std::fs;

use c1::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (mut left, mut right): (Vec<usize>, Vec<usize>) = parse_input(input);

    let mut distances: Vec<usize> = vec![];

    left.sort();
    right.sort();
    right.reverse();

    for x in left.into_iter() {
        if let Some(y) = right.pop() {
            distances.push(x.abs_diff(y));
        }
    }

    let result = distances.into_iter().reduce(|acc, n| acc + n);

    println!("{}", result.unwrap());
}
