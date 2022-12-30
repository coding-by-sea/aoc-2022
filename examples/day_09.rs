use aoc_2022::day_09::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_09").unwrap();
    let moves = parsing(&lines);
    println!("part 1: {}", part_1(&moves));
    println!("part 2: {}", part_2(&moves));
}