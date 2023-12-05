use std::io;

use aoc2023_04_2::num_cards;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = num_cards(lines);
    println!("{answer}");
}
