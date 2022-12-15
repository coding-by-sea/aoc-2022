use std::collections::{HashSet, VecDeque};
use std::convert::From;
use anyhow::{anyhow, Result, Error};

const NUM_START: u8 = 83;
const ELEVATION_START: u8 = 97;
const NUM_END: u8 = 69;
const ELEVATION_END: u8 = 122;
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];


pub fn parsing(lines: Vec<String>) -> Vec<Vec<u8>> {
    lines.into_iter().map(|line| line.into_bytes()).collect()
}

pub fn part_1(grid: &Vec<Vec<u8>>) -> usize {
    let num_row = grid.len() as i32;
    let num_col = grid[0].len() as i32;
    let mut start_loc = (0, 0);
    let mut end_loc= (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if *num == NUM_START {
                start_loc = (i as i32, j as i32);
            }
            if *num == NUM_END {
                end_loc = (i as i32, j as i32);
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_loc, 0));
    visited.insert(start_loc);
    loop {
        let (curr, mut step) = queue.pop_front().unwrap();
        if curr == end_loc {
            return step;
        }
        for direction in DIRECTIONS {
            let new_pos = (curr.0 + direction.0, curr.1 + direction.1);
            if visited.contains(&new_pos) {
                continue;
            }
            if is_valid_move(grid, num_row, num_col, new_pos, curr, |new_elevation, elevation| new_elevation <= elevation + 1) {
                queue.push_back((new_pos, step + 1));
                visited.insert(new_pos);
            }
        }
    }
}

fn is_valid_move(grid: &Vec<Vec<u8>>, num_row: i32, num_col: i32, new_pos: (i32, i32), pos: (i32, i32), cmp: fn(u8, u8) -> bool ) -> bool {
    if new_pos.0 < 0 || new_pos.0 >= num_row || new_pos.1 < 0 || new_pos.1 >= num_col {
        return false
    }
    let mut new_elevation = grid[new_pos.0 as usize][new_pos.1 as usize];
    if new_elevation == NUM_END {
        new_elevation = ELEVATION_END;
    }
    if new_elevation == NUM_START {
        new_elevation = ELEVATION_START;
    }
    let mut elevation = grid[pos.0 as usize][pos.1 as usize];
    if elevation == NUM_START {
        elevation = ELEVATION_START;
    }
    if elevation == NUM_END {
        elevation = ELEVATION_END;
    }
    cmp(new_elevation, elevation)
}


pub fn part_2(grid: &Vec<Vec<u8>>) -> u32 {
    let num_row = grid.len() as i32;
    let num_col = grid[0].len() as i32;
    let mut start_loc= (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            if *num == NUM_END {
                start_loc = (i as i32, j as i32);
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start_loc, 0));
    visited.insert(start_loc);
    loop {
        let (curr, mut step) = queue.pop_front().unwrap();
        if grid[curr.0 as usize][curr.1 as usize] == NUM_START || grid[curr.0 as usize][curr.1 as usize] == ELEVATION_START {
            return step;
        }
        for direction in DIRECTIONS {
            let new_pos = (curr.0 + direction.0, curr.1 + direction.1);
            if visited.contains(&new_pos) {
                continue;
            }
            if is_valid_move(grid, num_row, num_col, new_pos, curr,  |new_elevation, elevation| new_elevation >= elevation - 1) {
                queue.push_back((new_pos, step + 1));
                visited.insert(new_pos);
            }
        }
    }
}