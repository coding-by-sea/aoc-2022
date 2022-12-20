use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::{anyhow, Result, Error};
use regex::Regex;

const MINUTES: usize = 30;

pub fn parsing(lines: &Vec<String>) -> (HashMap<String, Vec<String>>, HashMap<String, usize>) {
    let mut valves_connection_mapping = HashMap::new();
    let mut valve_flow_rate_mapping = HashMap::new();
    for line in lines {
        let re = Regex::new(r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        let caps = re.captures(line).unwrap();
        valve_flow_rate_mapping.insert(caps[1].to_owned(), usize::from_str(&caps[2]).unwrap());
        valves_connection_mapping.insert(caps[1].to_owned(), caps[3].split(", ").map(|x| x.to_owned()).collect());
    }
    (valves_connection_mapping, valve_flow_rate_mapping)
}


fn dfs(
    best_records: &mut HashMap<(String, usize), usize>,
    mins_left: usize,
    pos: String,
    mut opened_valves: HashSet<String>,
    pressure_records: &mut Vec<usize>,
    pressure_released: usize,
    valves_connection_mapping: &HashMap<String, Vec<String>>,
    valve_flow_rate_mapping: &HashMap<String, usize>,
    num_valves_with_flow: usize,
) {
    let record = best_records.entry((pos.clone(), mins_left)).or_insert(pressure_released);
    if *record <= pressure_released {
        *record = pressure_released;
    }
    else {
        return;
    }
    if mins_left == 0 || mins_left - 1 == 0 {
        pressure_records.push(pressure_released);
        return
    }
    if opened_valves.len() == num_valves_with_flow {
        pressure_records.push(pressure_released);
        return
    }

    for neighbor in &valves_connection_mapping[&pos] {
        dfs(best_records, mins_left-1, neighbor.to_owned(), opened_valves.clone(), pressure_records, pressure_released, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    }

    if valve_flow_rate_mapping[&pos] == 0 {
        return;
    }
    if opened_valves.contains(&pos) {
        return;
    }
    else {
        opened_valves.insert(pos.clone());
    }

    for neighbor in &valves_connection_mapping[&pos] {
        dfs(best_records, mins_left-2, neighbor.to_owned(), opened_valves.clone(), pressure_records, pressure_released + valve_flow_rate_mapping[&pos] * (mins_left-1), valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    }
}

pub fn part_1(valves_connection_mapping: &HashMap<String, Vec<String>>, valve_flow_rate_mapping: &HashMap<String, usize>) -> usize {
    let num_valves_with_flow = valve_flow_rate_mapping
        .values()
        .map(|x| {
            if *x > 0 {
                1
            }
            else {
                0
            }
        })
        .sum();
    let mut pressure_records = vec![];
    let mut best_records = HashMap::new();
    dfs(&mut best_records, MINUTES, "AA".to_owned(), HashSet::new(),  &mut pressure_records, 0, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    pressure_records.into_iter().max().unwrap()
}
pub fn part_2(valves_connection_mapping: HashMap<String, Vec<String>>, valve_flow_rate_mapping: HashMap<String, usize>) -> usize {
    0
}
