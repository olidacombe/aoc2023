use std::io;

use aoc2023_04_1::sum_points;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_points(lines);
    println!("{answer}");
}
