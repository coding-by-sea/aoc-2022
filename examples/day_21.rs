use aoc_2022::day_21::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_21").unwrap();
    let map = parsing(&lines);
    println!("part 1: {}", part_1(map.clone()));
    println!("part 2: {}", part_2(map.clone()));
}