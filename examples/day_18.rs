use aoc_2022::day_18::{parsing_cubes, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_18").unwrap();
    let cubes = parsing_cubes(&lines);
    println!("part 1: {}", part_1(&cubes));
    println!("part 2: {}", part_2(&cubes));
}