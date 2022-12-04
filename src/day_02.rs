use std::str::FromStr;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Error;
    fn from_str(s: &str)-> Result<Self> {
        match s {
            "A"|"X" => Ok(Shape::Rock),
            "B"|"Y" => Ok(Shape::Paper),
            "C"|"Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("Shape parsing error"))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Play {
    opponent: Shape,
    response: Shape,
}

impl FromStr for Play {
    type Err = Error;
    fn from_str(s: &str)-> Result<Self> {
        let (str_1, str_2) = s.split_once(" ").ok_or(anyhow!("Play parsing error"))?;
        let shape_1 = Shape::from_str(str_1)?;
        let shape_2 = Shape::from_str(str_2)?;
        Ok(Play {
            opponent: shape_1,
            response: shape_2,
        })
    }
}

impl Play {
    fn get_score(&self) -> i32 {
        match self.response {
            Shape::Rock => {
                1 + match self.opponent {
                    Shape::Rock => 3,
                    Shape::Paper => 0,
                    Shape::Scissors => 6,
                }
            },
            Shape::Paper => {
                2 + match self.opponent {
                    Shape::Rock => 6,
                    Shape::Paper => 3,
                    Shape::Scissors => 0,
                }
            },
            Shape::Scissors => {
                3 + match self.opponent {
                    Shape::Rock => 0,
                    Shape::Paper => 6,
                    Shape::Scissors => 3,
                }
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = Error;
    fn from_str(s: &str)-> Result<Self> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Outcome parsing error"))
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Shape,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = Error;
    fn from_str(s: &str)-> Result<Self> {
        let (str_1, str_2) = s.split_once(" ").ok_or(anyhow!("Play parsing error"))?;
        let shape = Shape::from_str(str_1)?;
        let outcome = Outcome::from_str(str_2)?;
        Ok(Round {
            opponent: shape,
            outcome,
        })
    }
}

impl Round {
    fn get_score(&self) -> i32 {
        match self.outcome {
            Outcome::Lose => {
                0 + match self.opponent {
                    Shape::Rock => 3,
                    Shape::Paper => 1,
                    Shape::Scissors => 2,
                }
            },
            Outcome::Draw => {
                3 + match self.opponent {
                    Shape::Rock => 1,
                    Shape::Paper => 2,
                    Shape::Scissors => 3,
                }
            },
            Outcome::Win => {
                6 + match self.opponent {
                    Shape::Rock => 2,
                    Shape::Paper => 3,
                    Shape::Scissors => 1,
                }
            },
        }
    }
}

pub fn part_1(lines: &Vec<String>) -> i32 {
    lines.into_iter().map(|line| {
        let play = Play::from_str(line).unwrap();
        play.get_score()
    }).sum()
}

pub fn part_2(lines: &Vec<String>) -> i32 {
    lines.into_iter().map(|line| {
        Round::from_str(line).unwrap().get_score()
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_02::*;

    #[test]
    fn shape_parser_works() {
        let str = "A";
        assert_eq!(Shape::from_str(str).unwrap(), Shape::Rock);
    }
    #[test]
    fn play_parser_works() {
        let str = "A Y";
        assert_eq!(
            Play::from_str(str).unwrap(),
            Play {
                opponent: Shape::Rock,
                response: Shape::Paper,
            }
        )
    }
}