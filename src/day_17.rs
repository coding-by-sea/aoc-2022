use std::array::IntoIter;
use std::cmp::max;
use std::iter::{Cycle, Take};
use std::str::FromStr;

use anyhow::{bail, Error, Result};

const HORIZONTAL: [u8; 1] = [0b0011110];
const CROSS: [u8; 3] = [0b0001000, 0b0011100, 0b0001000];
const MIRROREDL: [u8; 3] = [0b0000100, 0b0000100, 0b0011100];
const VERTICAL: [u8; 4] = [0b0010000, 0b0010000, 0b0010000, 0b0010000];
const SQUARE: [u8; 2] = [0b0011000, 0b0011000];
const GRID_WIDTH: usize = 7;

fn move_array_left(array: &mut [u8]) {
    for num in array {
        *num = *num << 1;
    }
}
fn move_array_right(array: &mut [u8]) {
    for num in array {
        *num = *num >> 1;
    }
}

fn mark_grid(array: &[u8], grid: &mut Vec<u8>, height: usize) {
    for (i, row) in array.iter().rev().enumerate() {
        grid[height+i] = grid[height+i] | *row
    }
}


fn _collides(array: &[u8], grid: & Vec<u8>, height: usize) -> bool {
    for (i, row) in array.iter().rev().enumerate() {
        if grid[height+i] & *row > 0 {
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape{
    Horizontal([u8; 1]),
    Cross([u8; 3]),
    MirroredL([u8; 3]),
    Vertical([u8; 4]),
    Square([u8; 2]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rock {
    shape: Shape,
    height: usize,
    space_left_side: usize,
}

impl Rock {
    fn new(shape: Shape, height: usize, space_left_side: usize) -> Self {
        Rock {
            shape,
            height,
            space_left_side,
        }
    }

    fn moves_down(&mut self) {
        self.height -= 1;
    }

    fn move_left(&mut self) -> bool {
        if self.space_left_side <= 0 {
            return false;
        }
        match self.shape {
            Shape::Horizontal(ref mut array) => {move_array_left(array)},
            Shape::Cross(ref mut array) => {move_array_left(array)},
            Shape::MirroredL(ref mut array) => {move_array_left(array)},
            Shape::Vertical(ref mut array) => {move_array_left(array)},
            Shape::Square(ref mut array) => {move_array_left(array)},
        }
        self.space_left_side -= 1;
        true
    }
    fn move_right(&mut self) -> bool {
        if GRID_WIDTH - self.space_left_side - self.shape.width() <= 0 {
            return false;
        }
        match self.shape {
            Shape::Horizontal(ref mut array) => {move_array_right(array)},
            Shape::Cross(ref mut array) => {move_array_right(array)},
            Shape::MirroredL(ref mut array) => {move_array_right(array)},
            Shape::Vertical(ref mut array) => {move_array_right(array)},
            Shape::Square(ref mut array) => {move_array_right(array)},
        }
        self.space_left_side += 1;
        true
    }

    fn collides(&self, grid: &Vec<u8> ) -> bool {
        match self.shape {
            Shape::Horizontal(ref array) => {_collides(array, grid, self.height)},
            Shape::Cross(ref array) => {_collides(array, grid, self.height)},
            Shape::MirroredL(ref array) => {_collides(array, grid, self.height)},
            Shape::Vertical(ref array) => {_collides(array, grid, self.height)},
            Shape::Square(ref array) => {_collides(array, grid, self.height)},
        }
    }


    fn update_grid(&self, grid: &mut Vec<u8>) {
        match self.shape {
            Shape::Horizontal(ref array) => {mark_grid(array, grid, self.height)},
            Shape::Cross(ref array) => {mark_grid(array, grid, self.height)},
            Shape::MirroredL(ref array) => {mark_grid(array, grid, self.height)},
            Shape::Vertical(ref array) => {mark_grid(array, grid, self.height)},
            Shape::Square(ref array) => {mark_grid(array, grid, self.height)},
        }
    }
}

impl Shape {
    fn height(&self) -> usize {
        match self {
            Shape::Horizontal(array) => array.len(),
            Shape::Cross(array) => array.len(),
            Shape::MirroredL(array) => array.len(),
            Shape::Vertical(array) => array.len(),
            Shape::Square(array) => array.len(),
        }
    }
    fn width(&self) -> usize {
        match self {
            Shape::Horizontal(_) => 4,
            Shape::Cross(_) => 3,
            Shape::MirroredL(_) => 3,
            Shape::Vertical(_) => 1,
            Shape::Square(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Jet{
    Left,
    Right,
}

impl FromStr for Jet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == ">" {
            Ok(Jet::Right)
        }
        else if s == "<" {
            Ok(Jet::Left)
        }
        else {
            bail!("undefined direction")
        }
    }
}

pub fn parsing_jets(lines: &Vec<String>) -> Vec<Jet> {
    lines[0]
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| Jet::from_str(s).unwrap())
        .collect()
}


#[allow(dead_code)]
fn print_grid(grid: &Vec<u8>) {
    let base: u8 = 2;
    for row in grid.iter().rev() {
        for mask in (0..8).rev() {
            if *row & base.pow(mask) == base.pow(mask) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn get_rock_iterator(num: usize) -> Take<Cycle<IntoIter<Shape, 5>>> {
    [
        Shape::Horizontal(HORIZONTAL.clone()),
        Shape::Cross(CROSS.clone()),
        Shape::MirroredL(MIRROREDL.clone()),
        Shape::Vertical(VERTICAL.clone()),
        Shape::Square(SQUARE.clone()),
    ].into_iter().cycle().take(num)
}

pub fn part_1(jets: & Vec<Jet>) -> usize {
    let mut jets_iterator = jets.iter().cycle();
    let mut grid: Vec<u8> = vec![];
    let mut highest = 0;
    for shape in get_rock_iterator(2022) {
        let start_height = highest + 3;
        while grid.len() < start_height + shape.height() {
            grid.push(0);
        }
        let mut rock = Rock::new(
            shape,
            start_height, // the bottom
            2,
        );
        let mut still_falling = true;
        while still_falling {
            let jet = jets_iterator.next().unwrap();
            let mut rock_clone = rock.clone();
            match jet {
                Jet::Left => {
                    let moved = rock_clone.move_left();
                    if moved && !rock_clone.collides(&grid) {
                        rock = rock_clone;
                    }
                }
                Jet::Right => {
                    let moved = rock_clone.move_right();
                    if moved && !rock_clone.collides(&grid) {
                        rock = rock_clone;
                    }
                }
            }
            if rock.height == 0 {
                still_falling = false;
                rock.update_grid(&mut grid);
                highest = max(rock.height + rock.shape.height(), highest);
            }
            else {
                let mut rock_clone = rock.clone();
                rock_clone.moves_down();
                if !rock_clone.collides(&grid) {
                    rock = rock_clone;
                }
                else {
                    still_falling = false;
                    rock.update_grid(&mut grid);
                    highest = max(rock.height + rock.shape.height(), highest);
                }
            }
            }
        }
    highest
}
pub fn part_2(jets: & Vec<Jet>) -> usize{
    let mut jets_iterator = jets.iter().cycle();
    let mut grid: Vec<u8> = vec![];
    let mut highest = 0;
    let mut highest_increments = vec![];
    for shape in get_rock_iterator(2022) {
        let start_height = highest + 3;
        while grid.len() < start_height + shape.height() {
            grid.push(0);
        }
        let mut rock = Rock::new(
            shape,
            start_height, // the bottom
            2,
        );
        let mut still_falling = true;
        while still_falling {
            let jet = jets_iterator.next().unwrap();
            let mut rock_clone = rock.clone();
            match jet {
                Jet::Left => {
                    let moved = rock_clone.move_left();
                    if moved && !rock_clone.collides(&grid) {
                        rock = rock_clone;
                    }
                }
                Jet::Right => {
                    let moved = rock_clone.move_right();
                    if moved && !rock_clone.collides(&grid) {
                        rock = rock_clone;
                    }
                }
            }
            if rock.height == 0 {
                still_falling = false;
                rock.update_grid(&mut grid);
                let new_highest = max(rock.height + rock.shape.height(), highest);
                highest_increments.push(new_highest - highest);
                highest = new_highest;

            }
            else {
                let mut rock_clone = rock.clone();
                rock_clone.moves_down();
                if !rock_clone.collides(&grid) {
                    rock = rock_clone;
                }
                else {
                    still_falling = false;
                    rock.update_grid(&mut grid);
                    let new_highest = max(rock.height + rock.shape.height(), highest);
                    highest_increments.push(new_highest - highest);
                    highest = new_highest;
                }
            }
            }
        }
    println!("{:?}", highest_increments);
    0
}

