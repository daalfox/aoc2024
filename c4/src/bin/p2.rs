use std::{env, fs};

use c4::parse_input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();

    let (canvas, cols, rows) = parse_input(input);

    let mut result = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            // center should always be 'A'
            if canvas[r][c] != 'A' {
                continue;
            }

            match canvas[r - 1][c - 1] {
                'M' => {
                    if canvas[r + 1][c + 1] != 'S' {
                        continue;
                    }
                }
                'S' => {
                    if canvas[r + 1][c + 1] != 'M' {
                        continue;
                    }
                }
                _ => continue,
            }

            match canvas[r - 1][c + 1] {
                'M' => {
                    if canvas[r + 1][c - 1] != 'S' {
                        continue;
                    }
                }
                'S' => {
                    if canvas[r + 1][c - 1] != 'M' {
                        continue;
                    }
                }
                _ => continue,
            }

            result += 1;
        }
    }

    println!("{result}");
}
