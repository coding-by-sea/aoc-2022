use aoc_2022::day_01::{part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_01").unwrap();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));
}