use std::io;

use aoc2023_11_1::sum_of_shortest_paths;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_of_shortest_paths(lines);
    println!("Answer: {answer}");
}
