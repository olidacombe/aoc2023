use std::io;

use aoc2023_20_1::low_pulses_times_high_pulses_1k;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = low_pulses_times_high_pulses_1k(lines);
    println!("Answer: {answer}");
}
