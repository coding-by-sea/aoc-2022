use aoc_2022::day_22::{part_1, part_2, parsing};
use aoc_2022::utils;

fn main() {
    let lines = utils::read_lines("inputs/day_22.example").unwrap();
    let (board, instructions) = parsing(&lines);
    println!("{:?}", board);
    println!("{:?}", instructions);
    // println!("part 1: {}", part_1(&lines));
    // println!("part 2: {}", part_2(&lines));
}