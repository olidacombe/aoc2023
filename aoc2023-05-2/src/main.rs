use std::io;

use aoc2023_05_2::nearest_seed_location;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = nearest_seed_location(lines);
    println!("Answer: {answer}");
}
