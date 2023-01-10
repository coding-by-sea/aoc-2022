use std::cell::Cell;
use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{Error, Result, anyhow};

#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
pub struct Job {
    dependencies: Vec<String>,
    num_unsatisfied_dependencies: Cell<u8>,
    operation: Option<Operation>,
    value: Cell<Option<i64>>,
}


fn parse_single_line(line: &str) -> (String, Job) {
    let job;
    let (name, task) = line.split_once(": ").unwrap();
    if task.contains("+") {
        let (m, n) = task.split_once(" + ").unwrap();
        job = Job {
            dependencies: vec![m.to_owned(), n.to_owned()],
            num_unsatisfied_dependencies: Cell::new(2),
            operation: Some(Operation::Plus),
            value: Cell::new(None),
        }
    }
    else if task.contains("-") {
        let (m, n) = task.split_once(" - ").unwrap();
        job = Job {
            dependencies: vec![m.to_owned(), n.to_owned()],
            num_unsatisfied_dependencies: Cell::new(2),
            operation: Some(Operation::Minus),
            value: Cell::new(None),
        }
    }
    else if task.contains("*") {
        let (m, n) = task.split_once(" * ").unwrap();
        job = Job {
            dependencies: vec![m.to_owned(), n.to_owned()],
            num_unsatisfied_dependencies: Cell::new(2),
            operation: Some(Operation::Multiply),
            value: Cell::new(None),
        }
    }
    else if task.contains("/") {
        let (m, n) = task.split_once(" / ").unwrap();
        job = Job {
            dependencies: vec![m.to_owned(), n.to_owned()],
            num_unsatisfied_dependencies: Cell::new(2),
            operation: Some(Operation::Divide),
            value: Cell::new(None),
        }
    }
    else {
        job = Job {
            dependencies: vec![],
            num_unsatisfied_dependencies: Cell::new(0),
            operation: None,
            value: Cell::new(Some(i64::from_str(task).unwrap())),
        }
    }
    (name.to_owned(), job)
}


pub fn parsing(lines: &Vec<String>) -> HashMap<String, Job> {
    lines.iter().map(|x| parse_single_line(x)).collect()
}

pub fn part_1(map: HashMap<String, Job>) -> i64 {
    let dependency_map = get_dependency_map(&map);
    find_value_for_a_monkey(&map, &dependency_map, "root").unwrap()
}

fn get_dependency_map(map: &HashMap<String, Job>) -> HashMap<String, Vec<String>> {
    let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();
    for (name, job) in map.iter() {
        for dependency in job.dependencies.iter() {
            if dependency_map.contains_key(dependency) {
                dependency_map.get_mut(dependency).unwrap().push(name.to_owned());
            }
            else {
                dependency_map.insert(dependency.to_owned(), vec![name.to_owned()]);
            }
        }
    }
    dependency_map
}

fn find_value_for_a_monkey(map: &HashMap<String, Job>, dependency_map: &HashMap<String, Vec<String>>, target: &str) -> Result<i64> {
    let mut queue = vec![];
    for (name, job) in map.iter() {
        if job.num_unsatisfied_dependencies.get() == 0 {
            queue.push(name)
        }
    }
    while !queue.is_empty() {
        let dependency = queue.pop().unwrap();
        for name in dependency_map.get(dependency).unwrap().iter() {
            let job = map.get(name).unwrap();
            if job.num_unsatisfied_dependencies.get() == 0 {
                continue;
            }
            job.num_unsatisfied_dependencies.set(job.num_unsatisfied_dependencies.get()-1);
            if job.num_unsatisfied_dependencies.get() == 0 {
                let m = map.get(&job.dependencies[0]).unwrap().value.get().unwrap();
                let n = map.get(&job.dependencies[1]).unwrap().value.get().unwrap();
                let value = match job.operation {
                    None => {panic!("expect an operation")}
                    Some(Operation::Plus) => {m + n}
                    Some(Operation::Minus) => {m - n}
                    Some(Operation::Multiply) => {m * n}
                    Some(Operation::Divide) => {m / n}
                };
                job.value.replace(Some(value));
                queue.push(name);
                if name == target {
                    return Ok(value)
                }
            }
        }
    }
    Err(anyhow!("Cannot work out {}", target))
}


pub fn part_2(mut map: HashMap<String, Job>) -> i64 {
    map.remove("humn");
    let dependency_map = get_dependency_map(&map);
    let two_monkeys_in_root = map.get("root").unwrap().dependencies.clone();
    let matching_value;
    let relevant_monkey;
    match find_value_for_a_monkey(&map.clone(), &dependency_map, &two_monkeys_in_root[0]) {
        Ok(value) => {
            matching_value = value;
            relevant_monkey = two_monkeys_in_root[1].to_owned();
        }
        Err(_) => {
            matching_value = find_value_for_a_monkey(&map.clone(), &dependency_map, &two_monkeys_in_root[1]).unwrap();
            relevant_monkey = two_monkeys_in_root[0].to_owned();
        }
    }
    let _ = find_value_for_a_monkey(&map, &dependency_map, &relevant_monkey);
    map.get(&relevant_monkey).unwrap().value.set(Some(matching_value));
    map.insert(
        "humn".to_owned(),
        Job {
            dependencies: vec![],
            num_unsatisfied_dependencies: Cell::new(0),
            operation: None,
            value: Cell::new(None),
        }
    );
    let mut left_hand_name = relevant_monkey;
    loop {
        // println!("job target: {:?}", map.get(&target));
        // println!("target: {}", target);
        left_hand_name =  solve(&map, left_hand_name);
        if &left_hand_name == "humn" {
            return map.get("humn").unwrap().value.get().unwrap()
        }
    }
}

fn solve(map: &HashMap<String, Job>, left_hand_name: String) -> String {
    let job = map.get(&left_hand_name).unwrap();
    let left_hand_value = job.value.get().unwrap();
    let first_name = job.dependencies.first().unwrap();
    let second_name = job.dependencies.last().unwrap();
    match map.get(first_name).unwrap().value.get() {
        Some(value) => {
            let second_value;
            match job.operation {
                None => {
                    panic!("expected an operation")
                }
                Some(Operation::Plus) => {
                    second_value = left_hand_value - value;
                }
                Some(Operation::Minus) => {
                    second_value = value - left_hand_value;
                }
                Some(Operation::Multiply) => {
                    second_value = left_hand_value / value;
                }
                Some(Operation::Divide) => {
                    second_value = left_hand_value / value;
                }
            }
            map.get(second_name).unwrap().value.set(Some(second_value));
            second_name.to_owned()
        }
        None => {
            let value;
            let second_value = map.get(second_name).unwrap().value.get().unwrap();
            match job.operation {
                None => {
                    panic!("expected an operation")
                }
                Some(Operation::Plus) => {
                    value = left_hand_value - second_value;
                }
                Some(Operation::Minus) => {
                    value = second_value + left_hand_value;
                }
                Some(Operation::Multiply) => {
                    value = left_hand_value / second_value;
                }
                Some(Operation::Divide) => {
                    value = left_hand_value * second_value;
                }
            }
            map.get(first_name).unwrap().value.set(Some(value));
            first_name.to_owned()
        }
    }
}
