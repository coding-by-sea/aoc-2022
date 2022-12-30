use aoc_2022::day_02::{part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_02").unwrap();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));
}