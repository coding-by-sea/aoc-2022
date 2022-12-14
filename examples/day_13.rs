use aoc_2022::utils;
use aoc_2022::day_13::{part_1, part_2, parsing_pairs, parsing_packets};


fn main() {
    let lines = utils::read_lines("inputs/day_13").unwrap();
    let pairs = parsing_pairs(&lines);
    let packets= parsing_packets(&lines);
    println!("part 1: {}", part_1(&pairs));
    println!("part 2: {}", part_2(packets));
}