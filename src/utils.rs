use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_lines<P>(path: P) -> Result<Vec<String>, Error> where P: AsRef<Path> {
    BufReader::new(File::open(path)?).lines().collect()
}