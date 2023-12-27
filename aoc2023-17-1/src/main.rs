use std::io;

use aoc2023_17_1::minimum_heat_loss;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = minimum_heat_loss(lines);
    println!("Answer: {answer}");
}
