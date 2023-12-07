use std::io;

use aoc2023_07_2::total_winnings;

fn main() {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = total_winnings(lines);
    println!("Answer: {answer}");
}
