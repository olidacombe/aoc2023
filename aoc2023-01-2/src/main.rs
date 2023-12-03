use color_eyre::Result;
use std::io;

use aoc2023_01_2::sum_calibration;

fn main() -> Result<()> {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_calibration(lines)?;
    println!("{answer}");
    Ok(())
}
