use std::io;

use aoc2023_08_2::count_steps;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = count_steps(lines);
    println!("Answer: {answer}");
}
