use color_eyre::Result;
use std::io::{self};

use aoc2023_19_1::accepted_part_rating_sum;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::read_to_string(io::stdin())?;
    let answer = accepted_part_rating_sum(lines);
    println!("Answer: {answer}");
    Ok(())
}
