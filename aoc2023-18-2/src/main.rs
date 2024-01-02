use std::io;

use aoc2023_18_2::cubic_metres_of_lava;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = cubic_metres_of_lava(lines);
    println!("Answer: {answer}");
}
