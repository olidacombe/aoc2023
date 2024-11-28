use std::io;

use aoc2023_20_2::min_button_presses_to_trigger_rx;

fn main() {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = min_button_presses_to_trigger_rx(lines);
    println!("Answer: {answer}");
}
