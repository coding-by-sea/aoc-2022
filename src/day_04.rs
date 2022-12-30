use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug, PartialEq)]
struct IDRange(u64, u64);

impl IDRange {
    fn fully_contains(&self, other: &IDRange) -> bool {
        if self.0 <= other.0 && self.1 >= other.1 {
            return true;
        }
        return false;
    }
    fn overlaps(&self, other: &IDRange) -> bool {
        if self.0 > other.1 {
            return false;
        }
        else if self.1 < other.0 {
            return false;
        }
        return true;
    }
}

impl FromStr for IDRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_once("-").unwrap();
        Ok(IDRange(u64::from_str(left).unwrap(), u64::from_str(right).unwrap()))
    }
}

pub fn part_1(lines: &Vec<String>) -> u32 {
    let mut num_pairs = 0;
    for line in lines {
        let (s_1, s_2) = line.split_once(",").unwrap();
        let range_1 = IDRange::from_str(s_1).unwrap();
        let range_2 = IDRange::from_str(s_2).unwrap();
        if range_1.fully_contains(&range_2) || range_2.fully_contains(&range_1) {
            num_pairs += 1;
        }

    }
    num_pairs

}
pub fn part_2(lines: &Vec<String>) -> u32 {
    let mut num_pairs = 0;
    for line in lines {
        let (s_1, s_2) = line.split_once(",").unwrap();
        let range_1 = IDRange::from_str(s_1).unwrap();
        let range_2 = IDRange::from_str(s_2).unwrap();
        if range_1.overlaps(&range_2) {
            num_pairs += 1;
        }

    }
    num_pairs
}