use color_eyre::Result;
use std::io;

use aoc2023_02_2::sum_powers;

fn main() -> Result<()> {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_powers(lines);
    println!("{answer}");
    Ok(())
}
