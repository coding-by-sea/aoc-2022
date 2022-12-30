use aoc_2022::day_15::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_15").unwrap();
    let zones = parsing(lines);
    println!("part 1: {}", part_1(&zones));
    println!("part 2: {}", part_2(&zones));
}