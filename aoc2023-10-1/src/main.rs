use std::io;

use aoc2023_10_1::farthest_point;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = farthest_point(lines);
    println!("Answer: {answer}");
}
