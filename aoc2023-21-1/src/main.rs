use std::io;

use aoc2023_21_1::{number_of_reachable_garden_plots, Result};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = number_of_reachable_garden_plots(lines, 64)?;
    println!("Answer: {answer}");
    Ok(())
}
