use std::convert::From;
use anyhow::{anyhow, Result, Error};



pub fn parsing(lines: &Vec<String>) -> Vec<Vec<u32>> {
    lines.iter().map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect()
}

pub fn get_cumulative_maxes(grid: & Vec<Vec<u32>>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>, Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let n = grid.len();
    let mut left_to_right = grid.clone();
    for i in 0..n {
        for j in 1..n {
            if left_to_right[i][j] < left_to_right[i][j - 1] {
                left_to_right[i][j] = left_to_right[i][j - 1]
            }
        }
    }
    let mut right_to_left = grid.clone();
    for i in 0..n {
        for j in (0..n - 1).rev() {
            if right_to_left[i][j] < right_to_left[i][j + 1] {
                right_to_left[i][j] = right_to_left[i][j + 1]
            }
        }
    }
    let mut top_to_bottom = grid.clone();
    for j in 0..n {
        for i in 1..n {
            if top_to_bottom[i][j] < top_to_bottom[i - 1][j] {
                top_to_bottom[i][j] = top_to_bottom[i - 1][j]
            }
        }
    }
    let mut bottom_to_top = grid.clone();
    for j in 0..n {
        for i in (0..n - 1).rev() {
            if bottom_to_top[i][j] < bottom_to_top[i + 1][j] {
                bottom_to_top[i][j] = bottom_to_top[i + 1][j]
            }
        }
    }
    (left_to_right, right_to_left, top_to_bottom, bottom_to_top)
}
pub fn part_1(grid: &Vec<Vec<u32>>, left_to_right: &Vec<Vec<u32>>, right_to_left: &Vec<Vec<u32>>, top_to_bottom: &Vec<Vec<u32>>, bottom_to_top: &Vec<Vec<u32>>) -> usize {
    let n = grid.len();
    let mut num_interior_visible_notes = 0;
    for i in 1..n-1 {
        for j in 1..n-1 {
            if grid[i][j] > left_to_right[i][j-1] || grid[i][j] > right_to_left[i][j+1] || grid[i][j] > top_to_bottom[i-1][j] || grid[i][j] > bottom_to_top[i+1][j] {
                num_interior_visible_notes += 1;
            }
        }
    }
    n * 2 + (n - 2) * 2 + num_interior_visible_notes
}

pub fn part_2(grid: &Vec<Vec<u32>>) -> u32 {
    let n = grid.len();
    let mut highest_score = 0;
    for i in 1..(n-1) {
        for j in 1..(n-1) {
            let mut distance_left = 0;
            for l in (0..j).rev() {
                if grid[i][j] <= grid[i][l] || l == 0 {
                    distance_left = j - l;
                    break;
                }
            }
            let mut distance_right = 0;
            for r in j+1..n {
                if grid[i][j] <= grid[i][r] || r == n - 1 {
                    distance_right = r - j ;
                    break;
                }
            }
            let mut distance_top= 0;
            for t in (0..i).rev() {
                if grid[i][j] <= grid[t][j] || t == 0 {
                    distance_top = i - t;
                    break;
                }
            }
            let mut distance_down = 0;
            for d in i+1..n {
                if grid[i][j] <= grid[d][j] || d == n - 1 {
                    distance_down = d - i;
                    break;
                }
            }
            let score = (distance_left * distance_right * distance_top * distance_down) as u32;
            if score > highest_score {
                highest_score = score;
            }
            }
        }
    highest_score
}