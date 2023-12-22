use std::io;

use aoc2023_12_2::sum_possible_arrangements;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_possible_arrangements(lines);
    println!("Answer: {answer}");
}
