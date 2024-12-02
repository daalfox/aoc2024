use std::env;
use std::fs;

use c1::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (left, right): (Vec<usize>, Vec<usize>) = parse_input(input);

    let result = left.into_iter().fold(0, |acc, x| {
        acc + right
            .iter()
            .fold(0, |acc, &y| if y == x { acc + y } else { acc })
    });

    println!("{}", result);
}
