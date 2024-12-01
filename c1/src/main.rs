use std::env;
use std::fs;

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

fn parse_input(input: String) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let splitted: Vec<&str> = line.split("   ").collect();
        left.push(splitted[0].parse().unwrap());
        right.push(splitted[1].parse().unwrap());
    }

    (left, right)
}
