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
pub fn is_safe(report: &[usize]) -> bool {
    if !report.is_sorted_by(|a, b| a < b) && !report.is_sorted_by(|a, b| a > b) {
        return false;
    }

    report.is_sorted_by(|a, b| a.abs_diff(*b) <= 3)
}
