use std::io;

use aoc2023_10_2::num_enclosed_tiles;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = num_enclosed_tiles(lines);
    println!("Answer: {answer}");
}
