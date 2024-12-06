use std::{env, fs};

use c4::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (canvas, cols, rows) = parse_input(input);

    let mut result = 0;

    // horizontal
    for r in canvas.iter() {
        for c in 0..cols - 3 {
            if is_xmas([r[c], r[c + 1], r[c + 2], r[c + 3]]) {
                result += 1;
            }
        }
    }
    // horizontal reversed
    for r in canvas.iter() {
        for c in 3..cols {
            if is_xmas([r[c], r[c - 1], r[c - 2], r[c - 3]]) {
                result += 1;
            }
        }
    }
    // vertical
    for r in 0..rows - 3 {
        for c in 0..cols {
            if is_xmas([
                canvas[r][c],
                canvas[r + 1][c],
                canvas[r + 2][c],
                canvas[r + 3][c],
            ]) {
                result += 1;
            }
        }
    }
    // vertical reversed
    for r in 3..rows {
        for c in 0..cols {
            if is_xmas([
                canvas[r][c],
                canvas[r - 1][c],
                canvas[r - 2][c],
                canvas[r - 3][c],
            ]) {
                result += 1;
            }
        }
    }
    // tl to br
    for r in 0..rows - 3 {
        for c in 0..cols - 3 {
            if is_xmas([
                canvas[r][c],
                canvas[r + 1][c + 1],
                canvas[r + 2][c + 2],
                canvas[r + 3][c + 3],
            ]) {
                result += 1;
            }
        }
    }
    // br to tl
    for r in 3..rows {
        for c in 3..cols {
            if is_xmas([
                canvas[r][c],
                canvas[r - 1][c - 1],
                canvas[r - 2][c - 2],
                canvas[r - 3][c - 3],
            ]) {
                result += 1;
            }
        }
    }
    // tr to bl
    for r in 0..rows - 3 {
        for c in 3..cols {
            if is_xmas([
                canvas[r][c],
                canvas[r + 1][c - 1],
                canvas[r + 2][c - 2],
                canvas[r + 3][c - 3],
            ]) {
                result += 1;
            }
        }
    }
    // bl to tr
    for r in 3..rows {
        for c in 0..cols - 3 {
            if is_xmas([
                canvas[r][c],
                canvas[r - 1][c + 1],
                canvas[r - 2][c + 2],
                canvas[r - 3][c + 3],
            ]) {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn is_xmas(c: [char; 4]) -> bool {
    c.into_iter().collect::<String>() == "XMAS"
}
