use std::{env, fs};

use c3::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();
    let splitted: Vec<_> = file.split("do()").collect();

    let result = splitted.into_iter().fold(0, |acc, part| {
        let do_only = part.split("don't()").collect::<Vec<_>>()[0];

        let pairs: Vec<(usize, usize)> = parse_input(do_only);

        acc + pairs.into_iter().map(|pair| pair.0 * pair.1).sum::<usize>()
    });

    println!("{result}");
}
