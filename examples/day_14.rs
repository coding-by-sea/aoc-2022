use aoc_2022::utils;
use aoc_2022::day_14::{part_1, part_2, parsing_pairs};


fn main() {
    let lines = utils::read_lines("inputs/day_14").unwrap();
    let pairs = parsing_pairs(lines);
    println!("part 1: {}", part_1(&pairs));
    println!("part 2: {}", part_2(&pairs));
}