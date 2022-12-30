use aoc_2022::day_17::{parsing_jets, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_17").unwrap();
    let jets = parsing_jets(&lines);
    println!("part 1: {}", part_1(&jets));
    println!("part 2: {}", part_2(&jets)); // solve the problem by finding recurring patterns in the printed collection
}