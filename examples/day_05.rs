use aoc_2022::day_05::{parsing, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_05").unwrap();
    let (stacks, mut commands) = parsing(&lines);
    println!("part 1: {}", part_1(&mut stacks.clone(), &commands));
    println!("part 2: {}", part_2(&mut stacks.clone(), &commands));
}