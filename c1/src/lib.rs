pub fn parse_input(input: String) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let splitted: Vec<&str> = line.split("   ").collect();
        left.push(splitted[0].parse().unwrap());
        right.push(splitted[1].parse().unwrap());
    }

    (left, right)
}
