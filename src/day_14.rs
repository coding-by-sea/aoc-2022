use std::cmp::{max, min};
use std::str::FromStr;

const BORDER:i32 = -1;
const EMPTY:i32 = 0;
const ROCK:i32 = 1;
const SAND:i32 = 2;

pub fn parsing_pairs(lines: Vec<String>) -> Vec<((usize, usize), (usize, usize))> {
    let mut pairs = vec![];
    for line in lines.into_iter() {
        let nodes: Vec<&str> = line.as_str().split(" -> ").collect();
        for i in 0..(nodes.len()-1) {
            let (left_y, left_x) = nodes[i].split_once(",").unwrap();
            let (right_y, right_x) = nodes[i+1].split_once(",").unwrap();
            pairs.push(((usize::from_str(left_x).unwrap(), usize::from_str(left_y).unwrap()), (usize::from_str(right_x).unwrap(), usize::from_str(right_y).unwrap())));
        }
    }
    pairs
}

pub fn part_1(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    const Y_OFFSET:usize = 400;
    const GRID_SIZE:usize = 200;
    let mut grid = [[EMPTY; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        grid[i][0] = -1;
        grid[i][GRID_SIZE-1] = BORDER;
    }
    for j in 0..GRID_SIZE {
        grid[GRID_SIZE-1][j] = BORDER;
    }
    for pair in pairs {
        if pair.0.0 == pair.1.0 {
            let start = min(pair.0.1, pair.1.1);
            let end = max(pair.0.1, pair.1.1);
            for y in start..=end {
                grid[pair.0.0][y-Y_OFFSET] = ROCK;
            }
        }
        if pair.0.1 == pair.1.1 {
            let start = min(pair.0.0, pair.1.0);
            let end = max(pair.0.0, pair.1.0);
            for x in start..=end {
                grid[x][pair.0.1-Y_OFFSET] = ROCK;
            }
        }
    }
    let mut sand_flows_out = false;
    while sand_flows_out == false {
        let mut curr = (0, 100);
        loop {
            if grid[curr.0][curr.1] == BORDER {
                sand_flows_out = true;
                break
            }
            else if grid[curr.0 + 1][curr.1] == EMPTY || grid[curr.0 + 1][curr.1] == BORDER {
                curr.0 += 1;
            }
            else if grid[curr.0 + 1][curr.1 - 1] == EMPTY || grid[curr.0 + 1][curr.1 - 1] == BORDER {
                curr.0 += 1;
                curr.1 -= 1;
            }
            else if grid[curr.0 + 1][curr.1 + 1] == EMPTY || grid[curr.0 + 1][curr.1 + 1] == BORDER {
                curr.0 += 1;
                curr.1 += 1;
            }
            else {
                grid[curr.0][curr.1] = SAND;
                break
            }
        }
    }
    let mut num_sands = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid[i][j] == SAND {
                num_sands += 1
            }
        }
    }
    num_sands
}
pub fn part_2(pairs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    const Y_OFFSET:usize = 300;
    const GRID_SIZE:usize = 400;
    let highest = max(pairs.iter().map(|x| x.1.0).max().unwrap(), pairs[0].0.0);
    let mut grid = [[EMPTY; GRID_SIZE]; GRID_SIZE];
    for i in 0..GRID_SIZE {
        grid[i][0] = -1;
        grid[i][GRID_SIZE-1] = BORDER;
    }
    for j in 0..GRID_SIZE {
        grid[highest+2][j] = ROCK;
    }
    for pair in pairs {
        if pair.0.0 == pair.1.0 {
            let start = min(pair.0.1, pair.1.1);
            let end = max(pair.0.1, pair.1.1);
            for y in start..=end {
                grid[pair.0.0][y-Y_OFFSET] = ROCK;
            }
        }
        if pair.0.1 == pair.1.1 {
            let start = min(pair.0.0, pair.1.0);
            let end = max(pair.0.0, pair.1.0);
            for x in start..=end {
                grid[x][pair.0.1-Y_OFFSET] = ROCK;
            }
        }
    }
    let mut reaches_source= false;
    while reaches_source == false {
        let mut curr = (0, 200);
        loop {
            if grid[0][200] == SAND {
                reaches_source = true;
                break
            }
            else if grid[curr.0 + 1][curr.1] == EMPTY || grid[curr.0 + 1][curr.1] == BORDER {
                curr.0 += 1;
            }
            else if grid[curr.0 + 1][curr.1 - 1] == EMPTY || grid[curr.0 + 1][curr.1 - 1] == BORDER {
                curr.0 += 1;
                curr.1 -= 1;
            }
            else if grid[curr.0 + 1][curr.1 + 1] == EMPTY || grid[curr.0 + 1][curr.1 + 1] == BORDER {
                curr.0 += 1;
                curr.1 += 1;
            }
            else {
                grid[curr.0][curr.1] = SAND;
                break
            }
        }
    }
    let mut num_sands = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if grid[i][j] == SAND {
                num_sands += 1
            }
        }
    }
    num_sands
}
