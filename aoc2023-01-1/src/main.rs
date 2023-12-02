use std::io;

use aoc2023_01_1::get_line_calibration_value;

fn main() {
    let answer = io::stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| get_line_calibration_value(s.as_str()).ok())
        .reduce(|acc, v| acc + v)
        .unwrap();
    println!("{answer}");
}
