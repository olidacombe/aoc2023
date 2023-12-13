use std::io;

use aoc2023_12_1::sum_possible_arrangements;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_possible_arrangements(lines);
    println!("Answer: {answer}");
}
