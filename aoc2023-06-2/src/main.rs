use std::io;

use aoc2023_06_2::num_record_breaking_strategies;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = num_record_breaking_strategies(lines);
    println!("Answer: {answer}");
}
