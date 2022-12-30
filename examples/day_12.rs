use aoc_2022::day_12::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_12").unwrap();
    let grid = parsing(lines);
    println!("part 1: {}", part_1(&grid));
    println!("part 2: {}", part_2(&grid));
}