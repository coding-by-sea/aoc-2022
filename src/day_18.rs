use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;
use anyhow::{anyhow, Error, Result};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl FromStr for Cube {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iterator = s.split(",");
        Ok(Cube {
            x: i8::from_str(iterator.next().ok_or(anyhow!("not enough data"))?)?,
            y: i8::from_str(iterator.next().ok_or(anyhow!("not enough data"))?)?,
            z: i8::from_str(iterator.next().ok_or(anyhow!("not enough data"))?)?,
        })
    }
}

impl Cube {
    fn new(x: i8, y: i8, z: i8) -> Cube {
        Cube {
            x,
            y,
            z,
        }
    }

    fn get_neighbors(&self) -> [Self; 6] {
        [
            Cube::new(self.x+1, self.y, self.z),
            Cube::new(self.x-1, self.y, self.z),
            Cube::new(self.x, self.y+1, self.z),
            Cube::new(self.x, self.y-1, self.z),
            Cube::new(self.x, self.y, self.z+1),
            Cube::new(self.x, self.y, self.z-1),
        ]
    }
    fn is_out_of_bound(&self, x_min: i8, x_max: i8, y_min: i8, y_max: i8, z_min: i8, z_max: i8) -> bool {
        if self.x < x_min - 1 {return true};
        if self.x > x_max + 1 {return true};
        if self.y < y_min - 1 {return true};
        if self.y > y_max + 1 {return true};
        if self.z < z_min - 1 {return true};
        if self.z > z_max + 1 {return true};
        return false;
    }
}

pub fn parsing_cubes(lines: &Vec<String>) -> Vec<Cube> {
    lines.iter().map(|line| {Cube::from_str(line).unwrap()}).collect()
}

pub fn part_1(cubes: &Vec<Cube>) -> usize {
    let mut res = 0;
    let occupied: HashSet<&Cube> = cubes.iter().collect();
    for cube in cubes {
        for neighbor in cube.get_neighbors() {
            if !occupied.contains(&neighbor) {
                res += 1;
            }
        }
    }
    res
}



pub fn part_2(cubes: &Vec<Cube>) -> usize {
    let mut x_min = cubes[0].x;
    let mut x_max = cubes[0].x;
    let mut y_min = cubes[0].y;
    let mut y_max = cubes[0].y;
    let mut z_min = cubes[0].z;
    let mut z_max = cubes[0].z;
    for cube in cubes.iter().skip(1) {
        x_min = min(x_min, cube.x);
        x_max = max(x_max, cube.x);
        y_min = min(y_min, cube.y);
        y_max = max(y_max, cube.y);
        z_min = min(z_min, cube.z);
        z_max = max(z_max, cube.z);
    }
    let occupied: HashSet<&Cube> = cubes.iter().collect();
    let starting_cube = Cube::new(x_min, y_min, z_min);
    assert!(!occupied.contains(&starting_cube));
    let mut visited: HashSet<Cube> = HashSet::new();
    visited.insert(starting_cube.clone());
    let mut queue = vec![starting_cube];
    let mut res = 0;
    while queue.len() > 0 {
        let last_visited = queue.pop().unwrap();
        for neighbor in last_visited.get_neighbors() {
            if occupied.contains(&neighbor) {
                res += 1;
            }
            else if neighbor.is_out_of_bound(x_min, x_max, y_min, y_max, z_min, z_max) || visited.contains(&neighbor){}
            else {
                visited.insert(neighbor.clone());
                queue.push(neighbor);
            }
        }
    }
    res
}
