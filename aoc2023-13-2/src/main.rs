use std::io;

use aoc2023_13_2::reflection_summary;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = reflection_summary(lines);
    println!("Answer: {answer}");
}
