use std::str::FromStr;
use std::collections::HashSet;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, PartialEq)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: Direction,
    num_steps: i32,
}

impl FromStr for Move {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (direction_code, steps) = s.split_once(" ").unwrap();
        let num_steps= i32::from_str(steps).unwrap();
        let direction = match direction_code {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => Err(anyhow!("invalid direction code"))?
        };
        Ok(Move{
            direction,
            num_steps,
        })
    }
}

pub fn parsing(lines: &Vec<String>) -> Vec<Move> {
    lines.iter().map(|x| Move::from_str(x).unwrap()).collect()
}

fn get_new_tail_pos(new_head_pos: (i32, i32), curr_tail_pos: (i32, i32)) -> (i32, i32) {
    let x_diff = new_head_pos.0 - curr_tail_pos.0;
    let y_diff = new_head_pos.1 - curr_tail_pos.1;
    (curr_tail_pos.0 + x_diff.signum(), curr_tail_pos.1 + y_diff.signum())
}

fn is_adjacent(head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
    (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1
}

fn get_res(moves: & Vec<Move>, num_knots: usize) -> usize {
    let mut visited = HashSet::new();
    let mut knots_pos: Vec<_>= vec![(0, 0); num_knots];
    visited.insert(knots_pos[num_knots - 1]);
    for _move in moves {
        let head_loc_change = match _move.direction {
             Direction::U => (0, 1),
             Direction::D => (0, -1),
             Direction::L => (-1, 0),
             Direction::R => (1, 0),
        };
        for _ in 0.._move.num_steps {
            knots_pos[0].0 += head_loc_change.0;
            knots_pos[0].1 += head_loc_change.1;
            for i in 1..num_knots {
                while !is_adjacent(knots_pos[i-1], knots_pos[i]) {
                    knots_pos[i] = get_new_tail_pos(knots_pos[i-1], knots_pos[i]);
                    if i == num_knots - 1 {
                        visited.insert(knots_pos[i]);
                    }
                }
            }
        }
    }
   visited.len()
}

pub fn part_1(moves: & Vec<Move>) -> usize {
    get_res(moves, 2)
}

pub fn part_2(moves: & Vec<Move>) -> usize {
    get_res(moves, 10)
}