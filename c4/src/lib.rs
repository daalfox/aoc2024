pub fn parse_input(input: String) -> (Vec<Vec<char>>, usize, usize) {
    let canvas: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = canvas.len();
    let cols = canvas[0].len();

    (canvas, rows, cols)
}
