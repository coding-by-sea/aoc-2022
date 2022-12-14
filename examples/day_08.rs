use aoc_2022::utils;
use aoc_2022::day_08::{part_1, part_2, parsing, get_cumulative_maxes};


fn main() {
    let lines = utils::read_lines("inputs/day_08").unwrap();
    let grid = parsing(&lines);
    let (left_to_right, right_to_left, top_to_bottom, bottom_to_top) = get_cumulative_maxes(&grid);
    println!("part 1: {}", part_1(&grid, &left_to_right, &right_to_left, &top_to_bottom, &bottom_to_top));
    println!("part 2: {}", part_2(& grid));
}