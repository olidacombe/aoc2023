use std::io;

use aoc2023_16_2::max_num_energized_tiles;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = max_num_energized_tiles(lines);
    println!("Answer: {answer}");
}
