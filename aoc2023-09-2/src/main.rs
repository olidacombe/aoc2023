use std::io;

use aoc2023_09_2::extrapolated_sum;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = extrapolated_sum(lines);
    println!("Answer: {answer}");
}
