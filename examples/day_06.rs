use aoc_2022::utils;
use aoc_2022::day_06::{part_1, part_2};


fn main() {
    let lines = utils::read_lines("inputs/day_06").unwrap();
    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));
}