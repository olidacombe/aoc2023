use std::io;

use aoc2023_17_2::minimum_heat_loss;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = minimum_heat_loss(lines);
    println!("Answer: {answer}");
}
