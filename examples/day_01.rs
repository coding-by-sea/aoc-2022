use std::io::{BufRead, BufReader, Lines, Error};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

fn read_lines<P>(path: P) -> Result<Lines<BufReader<File>>, Error> where P: AsRef<Path> {
    Ok(BufReader::new(File::open(path)?).lines())
}

fn part_1() -> i32 {
    let mut maximum_calories = 0;
    let mut calories = 0;
    for line in read_lines("inputs/day_01").unwrap() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.as_str() == "" {
            if calories > maximum_calories {
                maximum_calories = calories;
            }
            calories = 0;
        }
        else {
            calories += i32::from_str(unwrapped_line.as_str()).unwrap();
        }
    }
    maximum_calories
}

fn part_2() -> i32 {
    let mut calories_collection = vec![];
    let mut calories = 0;
    for line in read_lines("inputs/day_01").unwrap() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.as_str() == "" {
            calories_collection.push(calories);
            calories = 0;
        }
        else {
            calories += i32::from_str(unwrapped_line.as_str()).unwrap();
        }
    }
    calories_collection.sort();
    calories_collection.reverse();
    calories_collection.into_iter().take(3).sum()
}

fn main() {
    println!("{}", part_2())
}