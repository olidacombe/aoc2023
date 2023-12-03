use std::io;

use aoc2023_03_1::sum_part_numbers;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_part_numbers(lines);
    println!("{answer}");
}
