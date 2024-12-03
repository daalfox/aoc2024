use std::{env, fs};

use c3::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let pairs: Vec<(usize, usize)> = parse_input(&input);

    let result: usize = pairs.into_iter().map(|pair| pair.0 * pair.1).sum();

    println!("{result}");
}
