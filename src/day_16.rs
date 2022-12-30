use std::collections::{HashMap, HashSet};
use std::str::FromStr;

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


fn dfs_1(
    best_records: &mut HashMap<(String, bool, usize), usize>,
    mins_left: usize,
    pos: String,
    mut opened_valves: HashSet<String>,
    pressure_records: &mut Vec<usize>,
    pressure_released: usize,
    valves_connection_mapping: &HashMap<String, Vec<String>>,
    valve_flow_rate_mapping: &HashMap<String, usize>,
    num_valves_with_flow: usize,
) {
    let cur_pos_opened;
    if opened_valves.contains(&pos) {
        cur_pos_opened = true;
    }
    else {
        cur_pos_opened = false;
    }
    if best_records.contains_key(&(pos.clone(), cur_pos_opened, mins_left)) && best_records[&(pos.clone(), cur_pos_opened, mins_left)] >= pressure_released {
        return;
    }
    else {
        best_records.insert((pos.clone(), cur_pos_opened, mins_left), pressure_released);
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
        dfs_1(best_records, mins_left-1, neighbor.to_owned(), opened_valves.clone(), pressure_records, pressure_released, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    }

    if valve_flow_rate_mapping[&pos] == 0 || opened_valves.contains(&pos) {
        return;
    }
    opened_valves.insert(pos.clone());
    dfs_1(best_records, mins_left-1, pos.to_owned(), opened_valves.clone(), pressure_records, pressure_released + valve_flow_rate_mapping[&pos] * (mins_left-1), valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
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
    dfs_1(&mut best_records, MINUTES, "AA".to_owned(), HashSet::new(),  &mut pressure_records, 0, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    pressure_records.into_iter().max().unwrap()
}

fn dfs_2(
    best_records: &mut HashMap<(String, String, bool, bool, usize), usize>,
    mins_left: usize,
    pos: String,
    epos: String, // elephant pos
    mut opened_valves: HashSet<String>,
    pressure_records: &mut Vec<usize>,
    pressure_released: usize,
    valves_connection_mapping: &HashMap<String, Vec<String>>,
    valve_flow_rate_mapping: &HashMap<String, usize>,
    num_valves_with_flow: usize,
) {
    let cur_pos_opened;
    if opened_valves.contains(&pos) {
        cur_pos_opened = true;
    }
    else {
        cur_pos_opened = false;
    }
    let cur_epos_opened;
    if opened_valves.contains(&epos) {
        cur_epos_opened = true;
    }
    else {
        cur_epos_opened = false;
    }
    // if released pressure is lower than the same positions at the same time, no need to continue on this path
    if best_records.contains_key(&(pos.clone(), epos.clone(), cur_pos_opened, cur_epos_opened, mins_left)) && best_records[&(pos.clone(), epos.clone(), cur_pos_opened, cur_epos_opened, mins_left)] >= pressure_released {
        return;
    }
    else {
        // pos and epos are mirrored
        best_records.insert((pos.clone(), epos.clone(), cur_pos_opened, cur_epos_opened, mins_left), pressure_released);
        best_records.insert((epos.clone(), pos.to_owned(), cur_epos_opened, cur_pos_opened, mins_left), pressure_released);
    }
    // not enough time to make a diff
    if mins_left <= 1 {
        pressure_records.push(pressure_released);
        return
    }
    // all valves which can release pressure have been opened
    if opened_valves.len() == num_valves_with_flow {
        pressure_records.push(pressure_released);
        return
    }

    // if neither choose to open valves
    for neighbor in &valves_connection_mapping[&pos] {
        for e_neighbor in &valves_connection_mapping[&epos] {
            dfs_2(best_records, mins_left - 1, neighbor.to_owned(), e_neighbor.to_owned(), opened_valves.clone(), pressure_records,  pressure_released, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
        }
    }

    // if only one choose to open valves
    let mut pos_opened_just_now = false;
    if valve_flow_rate_mapping[&pos] != 0 && !opened_valves.contains(&pos) {
        let mut opened_valves_clone = opened_valves.clone();
        opened_valves_clone.insert(pos.clone());
        pos_opened_just_now = true;
        for e_neighbor in &valves_connection_mapping[&epos] {
            dfs_2(best_records, mins_left - 1, pos.to_owned(), e_neighbor.to_owned(), opened_valves_clone.clone(), pressure_records,pressure_released + valve_flow_rate_mapping[&pos] * (mins_left-1), valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
        }
    }
    let mut epos_opened_just_now = false;
    if valve_flow_rate_mapping[&epos] != 0 && !opened_valves.contains(&epos) {
        let mut opened_valves_clone = opened_valves.clone();
        opened_valves_clone.insert(epos.clone());
        epos_opened_just_now = true;
        for neighbor in &valves_connection_mapping[&pos] {
            dfs_2(best_records, mins_left - 1, neighbor.to_owned(), epos.to_owned(), opened_valves_clone.clone(), pressure_records,pressure_released + valve_flow_rate_mapping[&epos] * (mins_left-1), valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
        }
    }
    // if both choose to open valves
    if pos != epos && pos_opened_just_now && epos_opened_just_now {
        opened_valves.insert(epos.clone());
        opened_valves.insert(pos.clone());
        dfs_2(best_records, mins_left - 1, pos.to_owned(), epos.clone(), opened_valves.clone(), pressure_records, pressure_released + valve_flow_rate_mapping[&pos] * (mins_left-1) + valve_flow_rate_mapping[&epos] * (mins_left-1), valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
        }
    }

pub fn part_2(valves_connection_mapping: &HashMap<String, Vec<String>>, valve_flow_rate_mapping: &HashMap<String, usize>) -> usize {
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
    dfs_2(&mut best_records, MINUTES-4, "AA".to_owned(), "AA".to_owned(),HashSet::new(), &mut pressure_records, 0, valves_connection_mapping, valve_flow_rate_mapping, num_valves_with_flow);
    pressure_records.into_iter().max().unwrap()
}