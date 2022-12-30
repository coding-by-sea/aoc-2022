use std::str::FromStr;

pub fn part_1(lines: &Vec<String>) -> i32 {
    let mut maximum_calories = 0;
    let mut calories = 0;
    for line in lines {
        if line == "" {
            if calories > maximum_calories {
                maximum_calories = calories;
            }
            calories = 0;
        }
        else {
            calories += i32::from_str(line).unwrap();
        }
    }
    maximum_calories
}

pub fn part_2(lines: &Vec<String>) -> i32 {
    let mut calories_collection = vec![];
    let mut calories = 0;
    for line in lines {
        if line == "" {
            calories_collection.push(calories);
            calories = 0;
        }
        else {
            calories += i32::from_str(line).unwrap();
        }
    }
    calories_collection.sort();
    calories_collection.reverse();
    calories_collection.into_iter().take(3).sum()
}