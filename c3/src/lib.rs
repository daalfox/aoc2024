use regex::Regex;

pub const PATTERN: &str = r"mul\((\d+),(\d+)\)";

pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(PATTERN).unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}
