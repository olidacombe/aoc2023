use std::{cmp, num::ParseIntError, ops::BitOr, sync::OnceLock};

use regex::Regex;
use thiserror::Error;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct GameRound {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl GameRound {
    pub fn red(mut self, red: u32) -> Self {
        self.red = red;
        self
    }
    pub fn green(mut self, green: u32) -> Self {
        self.green = green;
        self
    }
    pub fn blue(mut self, blue: u32) -> Self {
        self.blue = blue;
        self
    }
}

impl BitOr for GameRound {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::Output {
            red: cmp::max(self.red, rhs.red),
            green: cmp::max(self.green, rhs.green),
            blue: cmp::max(self.blue, rhs.blue),
        }
    }
}

#[derive(Error, Debug)]
pub enum GameRoundSpecError {
    #[error("Unreachable colour 🤷")]
    UnreachableColour(String),
    #[error("Unable to parse number of cubes")]
    CubeCount(#[from] ParseIntError),
}

impl TryFrom<&str> for GameRound {
    type Error = GameRoundSpecError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        static BALL_COUNT: OnceLock<Regex> = OnceLock::new();
        let matches = BALL_COUNT
            .get_or_init(|| Regex::new(r"\b(\d+)\s+(red|green|blue)\b").unwrap())
            .captures_iter(value)
            .map(|c| c.extract().1);

        for [num, colour] in matches {
            match colour {
                "red" => red = num.parse()?,
                "green" => green = num.parse()?,
                "blue" => blue = num.parse()?,
                unk => return Err(GameRoundSpecError::UnreachableColour(unk.to_string())),
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn lte(&self, max: &GameRound) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }
}

#[derive(Error, Debug)]
pub enum GameSpecError {
    #[error("Unable to parse game ID")]
    GameIdParse,
    #[error("Unable to parse game ID")]
    GameIdFormat(#[from] ParseIntError),
    #[error("Unable to parse rounds")]
    RoundsParse,
}

impl TryFrom<&str> for Game {
    type Error = GameSpecError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        static GAME_ID: OnceLock<Regex> = OnceLock::new();
        let id = GAME_ID
            .get_or_init(|| Regex::new(r"^Game (?<id>\d):").unwrap())
            .captures(value)
            .ok_or(GameSpecError::GameIdParse)?
            .name("id")
            .ok_or(GameSpecError::GameIdParse)?
            .as_str()
            .parse::<u32>()?;
        static GAME_ROUND: OnceLock<Regex> = OnceLock::new();
        let GameRound { red, green, blue } = GAME_ROUND
            // .get_or_init(|| Regex::new(r"[;:]([^;]+)[;$]").unwrap()) // this doesn't work
            // because `captures_iter` finds strictly non-overlapping matches
            .get_or_init(|| Regex::new(r"[;:]([^;]+)").unwrap())
            .captures_iter(value)
            .map(|c| (c.extract::<1>().1)[0])
            .map(GameRound::try_from)
            .filter_map(|r| r.ok())
            .reduce(|acc, v| acc | v)
            .ok_or(GameSpecError::RoundsParse)?;

        Ok(Self {
            id,
            red,
            green,
            blue,
        })
    }
}

pub fn sum_possible_ids(it: impl Iterator<Item = String>, max: &GameRound) -> u32 {
    it.filter_map(|line| Game::try_from(line.as_str()).ok())
        .filter(|game| game.lte(max))
        .fold(0, |acc, v| acc + v.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use color_eyre::Result;
    use indoc::indoc;

    #[test]
    fn round_deserialize() -> Result<()> {
        let round = GameRound::try_from(" 3 blue, 4 red")?;
        assert_eq!(
            round,
            GameRound {
                red: 4,
                green: 0,
                blue: 3
            }
        );
        Ok(())
    }

    #[test]
    fn round_or() {
        let round1 = GameRound {
            red: 10,
            green: 20,
            blue: 30,
        };
        let round2 = GameRound {
            red: 15,
            green: 15,
            blue: 20,
        };
        let round3 = round1 | round2;
        assert_eq!(
            round3,
            GameRound {
                red: 15,
                green: 20,
                blue: 30
            }
        );
    }

    #[test]
    fn game_deserialize() -> Result<()> {
        let game = Game::try_from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")?;
        assert_eq!(
            game,
            Game {
                id: 1,
                red: 4,
                green: 2,
                blue: 6
            }
        );
        Ok(())
    }

    #[test]
    fn full_calculation() {
        let example = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        assert_eq!(
            sum_possible_ids(
                example.lines().map(String::from),
                &GameRound::default().red(12).green(13).blue(14)
            ),
            8
        );
    }
}
