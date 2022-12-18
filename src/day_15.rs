use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use anyhow::{anyhow, Result, Error};
use regex::Regex;

#[derive(Debug)]
pub struct CoveredZone {
    sensor: (i32, i32),
    beacon: (i32, i32),
    radius: i32
}

impl FromStr for CoveredZone {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let sensor = (i32::from_str(&caps[1]).unwrap(), i32::from_str(&caps[2]).unwrap());
        let beacon = (i32::from_str(&caps[3]).unwrap(), i32::from_str(&caps[4]).unwrap());
        let radius = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        Ok(CoveredZone {
            sensor,
            beacon,
            radius
        })
    }
}
impl CoveredZone {
    fn is_relevant(&self, target_row: i32) -> bool {
        if target_row > self.sensor.1 + self.radius || target_row < self.sensor.1 - self.radius {
            return false;
        }
        return true;
    }
    fn get_overlap_range(&self, target_row: i32) -> (i32, i32) {
        let x_radius = self.radius - (target_row - self.sensor.1).abs();
        (self.sensor.0 - x_radius, self.sensor.0 + x_radius)
    }
}

pub fn parsing(lines: Vec<String>) -> Vec<CoveredZone> {
    lines.iter().map(|x| CoveredZone::from_str(x).unwrap()).collect()
}

pub fn part_1(zones: &Vec<CoveredZone>) -> i32 {
    let target_row = 2000000;
    let mut res = 0;
    for range in get_covered_range_in_one_row(zones, target_row) {
        res += range.1 - range.0 + 1
    }
    let mut visited_beacons = HashSet::new();
    for zone in zones {
        if zone.beacon.1 == target_row && !visited_beacons.contains(&zone.beacon) {
            res -= 1;
            visited_beacons.insert(zone.beacon);
        }
    }
    res
    }

fn get_covered_range_in_one_row(zones: &Vec<CoveredZone>, target_row: i32) -> Vec<(i32, i32)> {
    let mut ranges = vec!();
    for zone in zones {
        if zone.is_relevant(target_row) {
            ranges.push(zone.get_overlap_range(target_row))
        }
    }
    ranges.sort();
    if ranges.is_empty() {
        return vec![];
    }
    let mut merged_ranges = vec![ranges[0]];
    for range in ranges[1..].iter() {
        let previous_range = merged_ranges.pop().unwrap();
        if range.0 > previous_range.1 + 1 {
            merged_ranges.push(previous_range);
            merged_ranges.push(*range);
        } else {
            merged_ranges.push((previous_range.0, max(previous_range.1, range.1)));
        }
    }
    merged_ranges
}

pub fn part_2(zones: &Vec<CoveredZone>) -> i64 {
    let mut y = 0;
    for i in 0..=4000000 {
        let ranges = get_covered_range_in_one_row(zones, i);
        let mut fully_covered = false;
        for range in ranges {
            if range.1 >= 4000000 && range.0 <= 0 {
                fully_covered = true;
                break;
            }
        }
        if fully_covered == false {
            y = i;
            break;
        }
    }
    let ranges = get_covered_range_in_one_row(zones, y);
    let mut x = 0;
    for i in 1..ranges.len() {
        if ranges[i].0 - ranges[i-1].1 == 2 {
            x = ranges[i].0 - 1;
            if x >= 0 && x <= 4000000 {
                return x as i64 * 4000000 + y as i64;
            }
        }
    }
    0
}