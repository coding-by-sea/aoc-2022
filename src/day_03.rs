use std::str::FromStr;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, PartialEq)]
struct Bitmask(u64);

impl Bitmask {
    fn new(num: u8)-> Result<Self> {
        let mut num_shifts;
        let num_a = u8::try_from('a').unwrap();
        let num_z: u8 = num_a + 25;
        let num_A = u8::try_from('A').unwrap();
        let num_Z: u8 = num_A + 25;
        if num >= num_a && num <= num_z {
            num_shifts = num - num_a;
        }
        else if num >= num_A && num <= num_Z {
            num_shifts = num - num_A + 26;
        }
        else {
            return Err(anyhow!("{} is out of range", num));
        }
        Ok(Bitmask((1 as u64) << num_shifts))
    }
    fn add_bit(& mut self, other: &Bitmask) {
        self.0 = self.0 | other.0;
    }
    fn get_priorities(&self, other: &Bitmask) -> u32 {
        if (self.0 & other.0).count_ones() == 1 {
            other.0.trailing_zeros() + 1
        }
        else {
            0
        }
    }
    fn find_common_item(bitmasks: &mut Vec<Bitmask>) -> u32 {
        let mut res = Bitmask(u64::MAX);
        while let Some(Bitmask(num)) = bitmasks.pop() {
            res.0 = res.0 & num;
        }
        res.0.trailing_zeros() + 1
    }
}

pub fn part_1(lines: &Vec<String>) -> u32 {
    let mut priorities = 0;
    for line in lines {
        let mut bitmask = Bitmask(0);
        let bytes = line.as_bytes();
        assert_eq!(bytes.len() % 2, 0);
        for i in 0..bytes.len() {
            if i < bytes.len() / 2 {
                bitmask.add_bit(&Bitmask::new(bytes[i]).unwrap())
            }
            else {
                let priority = bitmask.get_priorities(&Bitmask::new(bytes[i]).unwrap());
                if priority > 0 {
                    priorities += priority;
                    break // the assumption is that there is only one type existing in both compartments
                }
            }
        }
    }
    priorities
}
pub fn part_2(lines: &Vec<String>) -> u32 {
    let mut priorities = 0;
    let mut bitmasks = vec![];
    for line in lines {
        let mut bitmask = Bitmask(0);
        let bytes = line.as_bytes();
        for i in 0..bytes.len() {
            bitmask.add_bit(&Bitmask::new(bytes[i]).unwrap())
            }
        bitmasks.push(bitmask);
        if bitmasks.len() == 3 {
            priorities += Bitmask::find_common_item(&mut bitmasks);
        }
    }
    priorities
}