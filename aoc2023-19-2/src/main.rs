use color_eyre::Result;
use std::io::{self};

use aoc2023_19_2::acceptable_parts_sum;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::read_to_string(io::stdin())?;
    let answer = acceptable_parts_sum(lines);
    println!("Answer: {answer}");
    Ok(())
}
