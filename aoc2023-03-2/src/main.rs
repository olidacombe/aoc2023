use std::io;

use aoc2023_03_2::sum_gear_ratios;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_gear_ratios(lines);
    println!("{answer}");
}
