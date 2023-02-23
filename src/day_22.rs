use std::str::FromStr;
#[derive(Debug, Clone)]
pub enum Tile {
    Open,
    Wall,
    OffLimit,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move(usize),
    TurnR,
    TurnL,
}

enum Direction {
    Up(usize, usize)
}

fn parsing_single_line_for_board(line: &String) -> Vec<Tile> {
    line.chars().map(|c| {
        match c {
            ' ' => Tile::OffLimit,
            '#' => Tile::Wall,
            '.' => Tile::Open,
            _ => {panic!("unexpected char")}
        }
    }
    ).collect()
}

fn parsing_single_line_for_instructions(line: &String) -> Vec<Instruction> {
    let mut nums = vec![];
    for num_str in line.split(|c| c == 'L' || c == 'R') {
        nums.push(usize::from_str(num_str).unwrap());
    }
    let LRs: Vec<char> = line.chars().filter(|c| *c == 'L' || *c == 'R').collect();
    let mut res = vec![];
    for i in 0..nums.len() {
        res.push(Instruction::Move(nums[i]));
        if i == nums.len() - 1 {
            break;
        }
        res.push(match LRs[i] {
            'L' => Instruction::TurnL,
            'R' => Instruction::TurnR,
            _ => {panic!("unexpected char")}
        });
    }
    res
}

pub fn parsing(lines: &Vec<String>) -> (Vec<Vec<Tile>>, Vec<Instruction>)  {
    let mut board = vec![];
    let mut lines_iterator = lines.iter();
    let mut parsing_board = true;
    while parsing_board {
        let line = lines_iterator.next().unwrap();
        if line.is_empty() {
            parsing_board = false;
            break;
        }
        board.push(parsing_single_line_for_board(line));
    }
    (board, parsing_single_line_for_instructions(lines_iterator.next().unwrap()))
}

pub fn part_1(lines: &Vec<String>) -> i32 {
    let mut maximum_calories = 0;
    let mut calories = 0;
    for line in lines {
        if line == "" {
            if calories > maximum_calories {
                maximum_calories = calories;
            }
            calories = 0;
        }
        else {
            calories += i32::from_str(line).unwrap();
        }
    }
    maximum_calories
}


pub fn part_2(lines: &Vec<String>) -> i32 {
    let mut calories_collection = vec![];
    let mut calories = 0;
    for line in lines {
        if line == "" {
            calories_collection.push(calories);
            calories = 0;
        }
        else {
            calories += i32::from_str(line).unwrap();
        }
    }
    calories_collection.sort();
    calories_collection.reverse();
    calories_collection.into_iter().take(3).sum()
}