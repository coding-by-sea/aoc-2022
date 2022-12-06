use std::str::FromStr;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, PartialEq)]
pub struct Command {
    from: usize,
    to: usize,
    num_crates: usize,
}

impl Command {

}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(" ");
        split.next(); // move
        let num_crates= usize::from_str(split.next().unwrap()).unwrap();
        split.next(); // from
        let from = usize::from_str(split.next().unwrap()).unwrap() - 1;
        split.next(); // to
        let to = usize::from_str(split.next().unwrap()).unwrap() - 1;
        Ok(Command {
            from,
            to,
            num_crates
        })
    }
}

pub fn parsing(lines: &Vec<String>) -> (Vec<String>, Vec<Command>) {
    let mut parsing_stacks = true;
    let mut stacks = vec![];
    let mut rev_stacks = vec![];
    let mut commands = vec![];
    let stop_words = [' ', '[', ']'];
    for line in lines {
        if line.starts_with(" 1") {
            continue
        }
        if line == "" {
            parsing_stacks = false;
            continue;
        }
        if parsing_stacks {
            for (i, char) in line.chars().enumerate() {
                if stop_words.contains(&char) {
                    continue
                }
                else {
                    let pos = i / 4;
                    while rev_stacks.len() <= pos {
                        rev_stacks.push("".to_string());
                    }
                    rev_stacks[pos].push(char);
                }
            }
        }
        else {
            commands.push(Command::from_str(line).unwrap());

        }
    }
    for stack in rev_stacks {
        stacks.push(stack.chars().rev().collect::<String>());
    }

    (stacks, commands)
}


pub fn part_1(stacks: &mut Vec<String>, commands: & Vec<Command>) -> String {
    let mut res = "".to_string();
    for command in commands {
        let mut num_crates = command.num_crates;
        while num_crates > 0 {
            match stacks[command.from].pop() {
                Some(char) => {
                    stacks[command.to].push(char);
                    num_crates -= 1;
                },
                None => break,
            }

        }
    }
    for stack in stacks {
        res.push(stack.pop().unwrap())
    }
    res
}

pub fn part_2(stacks: &mut Vec<String>, commands: & Vec<Command>) -> String {
    let mut res = "".to_string();
    for command in commands {
        let mut crates_to_move = "".to_string();
        let mut num_crates = command.num_crates;
        while num_crates > 0 {
            match stacks[command.from].pop() {
                Some(char) => {
                    crates_to_move.push(char);
                    num_crates -= 1;
                },
                None => break,
            }

        }
        while let Some(char) = crates_to_move.pop() {
            stacks[command.to].push(char);
        }

    }
    for stack in stacks {
        res.push(stack.pop().unwrap())
    }
    res
}