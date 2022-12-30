use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }
        else if s.starts_with("addx") {
            let (_, num_str) = s.split_once(" ").unwrap();
            return Ok(Instruction::Addx(i32::from_str(num_str).unwrap()));
        }
        else {
            return Err(anyhow!("invalid instruction"))
        }
    }
}

pub fn parsing(lines: &Vec<String>) -> Vec<Instruction> {
    lines.iter().map(|x| Instruction::from_str(x).unwrap()).collect()
}

pub fn part_1(instructions: & Vec<Instruction>) -> i32 {
    let mut cycle = 0;
    let mut x = 1;
    let targets = [20, 60, 100, 140, 180, 220];
    let mut sum_signals = 0;
    for instruction in instructions {
        let mut change = 0;
        let prev_cycle = cycle;
        match instruction {
            Instruction::Noop => {
                cycle += 1
            },
            Instruction::Addx(num) => {
                cycle += 2;
                change = *num;
            }
        }
        for target in targets {
            if target > prev_cycle && target <= cycle {
                sum_signals += x * target
            }
        }
        x += change;
    }
    sum_signals
}
fn mark_images(images: &mut [[&str; 40]; 6], x: i32, cycle: i32) {
    let row_idx = (cycle / 40) as usize;
    let col_idx = (cycle % 40) as usize;
    if x - 1 <= col_idx as i32 && x + 1 >= col_idx as i32 {
        images[row_idx][col_idx] = "#"
    }
}
pub fn part_2(instructions: & Vec<Instruction>) {
    let mut images = [["-"; 40]; 6];
    let mut cycle = 0;
    let mut x = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                mark_images(&mut images, x, cycle);
                cycle += 1;
            }
            Instruction::Addx(num) => {
                for _ in 0..2 {
                    mark_images(&mut images, x, cycle);
                    cycle += 1;
                }
                x += num;
            }
        }
        }
    for line in images {
        println!("{:?}", line.join(""));
    }

}
