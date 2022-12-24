use aoc_2022::utils;
use aoc_2022::day_16::{part_1, part_2, parsing};


fn main() {
    let lines = utils::read_lines("inputs/day_16").unwrap();
    let (connection_map, rate_map) = parsing(&lines);
    println!("part 1: {}", part_1(&connection_map, &rate_map));
    println!("part 2: {}", part_2(&connection_map, &rate_map));
}