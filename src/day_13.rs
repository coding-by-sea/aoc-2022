use std::cmp::{min, Ordering};

use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum PacketData {
    Integer(u8),
    List(Vec<PacketData>)
}

impl From<&[String]> for Pair {
    fn from(block: &[String]) -> Self {
        assert_eq!(block.len(), 2);
        Pair {
            left: serde_json::from_str(&block[0]).unwrap(),
            right: serde_json::from_str(&block[1]).unwrap(),
        }
    }
}

impl Pair {
    fn is_right_order(&self) -> bool {
        self.left < self.right
    }
}

impl PartialOrd<Self> for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&PacketData::Integer(ref num_left), &PacketData::Integer(ref num_right)) => {
                return num_left.cmp(num_right);
            }
            (&PacketData::List(ref nums_left), &PacketData::List(ref nums_right)) => {
                loop {
                    let min_length = min(nums_left.len(), nums_right.len());
                    for i in 0..min_length {
                        let order = nums_left[i].cmp(&nums_right[i]);
                        if order != Ordering::Equal {
                            return order;
                        }
                    }
                    return nums_left.len().cmp(&nums_right.len());
                }
            }
            (&PacketData::List(_), &PacketData::Integer(ref num_right)) => {
                let right = PacketData::List(vec![PacketData::Integer(*num_right)]);
                return self.cmp(&right);
            }
            (&PacketData::Integer(ref num_left), &PacketData::List(_)) => {
                let left = PacketData::List(vec![PacketData::Integer(*num_left)]);
                return left.cmp(other);
            }
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct Pair {
    left: PacketData,
    right: PacketData,
}

pub fn parsing_pairs(lines: &Vec<String>) -> Vec<Pair> {
    let mut res = vec![];
    for block in lines.split(|x| x.as_str() == "") {
        res.push(Pair::from(block))
    }
    res
}

pub fn parsing_packets(lines: &Vec<String>) -> Vec<PacketData> {
    lines.iter().filter(|x| !x.is_empty()).map(|x| serde_json::from_str(x).unwrap()).collect()
}

pub fn part_1(pairs: & Vec<Pair>) -> usize {
    let mut indices_right_order = vec![];
    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_right_order() {
            indices_right_order.push(i+1)
        }
    }
    indices_right_order.into_iter().sum()
}

pub fn part_2(packets: Vec<PacketData>) -> usize{
    let divider = vec![PacketData::List(vec![PacketData::List(vec![PacketData::Integer(2)])]), PacketData::List(vec![PacketData::List(vec![PacketData::Integer(6)])])];
    let mut all_packets = divider.clone();
    all_packets.extend(packets);
    all_packets.sort();
    all_packets.into_iter().enumerate().filter(|(_, packet)| *packet == divider[0] || *packet == divider[1]).map(|(i, _)| i + 1).product()
}

