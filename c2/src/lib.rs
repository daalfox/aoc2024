pub fn parse_input(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|level| level.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}
