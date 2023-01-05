use aoc_2022::day_20::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_20").unwrap();
    let nums = parsing(&lines);
    println!("part 1: {}", part_1(&nums));
    println!("part 2: {}", part_2(&nums));
}