use aoc_2022::day_11::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_11").unwrap();
    let monkeys = parsing(&lines);
    println!("part 1: {}", part_1(& monkeys.clone()));
    println!("part 2: {}", part_2(& monkeys.clone()));
}