use std::io;

use aoc2023_11_2::sum_of_shortest_paths;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_of_shortest_paths(lines, 1000000);
    println!("Answer: {answer}");
}
