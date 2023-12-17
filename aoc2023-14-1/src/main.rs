use std::io;

use aoc2023_14_1::total_load;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = total_load(lines);
    println!("Answer: {answer}");
}
