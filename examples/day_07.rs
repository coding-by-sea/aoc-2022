use aoc_2022::day_07::{get_directory_layout, part_1, part_2};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_07").unwrap();
    let map = get_directory_layout(&lines);
    println!("part 1: {}", part_1(&map));
    println!("part 2: {}", part_2(&map));
}