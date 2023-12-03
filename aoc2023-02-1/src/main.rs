use color_eyre::Result;
use std::io;

use aoc2023_02_1::{sum_possible_ids, GameRound};

fn main() -> Result<()> {
    let lines = io::stdin().lines().filter_map(|l| l.ok());
    let answer = sum_possible_ids(lines, &GameRound::default().red(12).green(13).blue(14));
    println!("{answer}");
    Ok(())
}
