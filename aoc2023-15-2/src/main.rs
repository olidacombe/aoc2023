use std::io;

use aoc2023_15_2::focusing_power;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = focusing_power(lines);
    println!("Answer: {answer}");
}
