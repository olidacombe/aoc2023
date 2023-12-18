use std::io;

use aoc2023_15_1::sum_hashes;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_hashes(lines);
    println!("Answer: {answer}");
}
