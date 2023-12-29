use std::io;

use aoc2023_18_1::cubic_meters_of_lava;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = cubic_meters_of_lava(lines);
    println!("Answer: {answer}");
}
