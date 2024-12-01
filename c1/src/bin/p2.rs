use std::env;
use std::fs;

use c1::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (left, right): (Vec<usize>, Vec<usize>) = parse_input(input);

    let mut score: Vec<usize> = vec![];

    for x in left.into_iter() {
        score.push(
            right
                .iter()
                .fold(0, |acc, &y| if y == x { acc + y } else { acc }),
        )
    }

    let result: usize = score.into_iter().sum();

    println!("{}", result);
}
