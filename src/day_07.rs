use std::cell::Cell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, Default)]
pub struct Node where {
    size: Cell<Option<usize>>,
    is_directory: bool,
    path: PathBuf,
    children: Vec<PathBuf>
}

impl Node {
    fn set_total_size(&self, path_node_map: & HashMap<PathBuf, Node>) -> usize {
        if let Some(size) = self.size.get() {
            return size;
        }
        let mut total_size = 0;
        for child_path in self.children.iter() {
            let child = path_node_map.get(child_path).unwrap();
            if let Some(size) = child.size.get() {
                total_size += size;
            }
            else {
                total_size += child.set_total_size(path_node_map);
            }
        }
        self.size.set(Some(total_size));
        total_size
    }
}

pub fn get_directory_layout(lines: &Vec<String>) -> HashMap<PathBuf, Node> {
    let mut path_node_map = HashMap::new();
    let mut root: Node = Node {
        size: Cell::new(None),
        is_directory: true,
        path: PathBuf::from("/"),
        children: vec![],
    };
    path_node_map.insert(PathBuf::from("/"), root);
    let mut curr_dir = PathBuf::from("/");
    for line in lines {
        if line.starts_with("$ cd") {
            match line.as_str() {
                "$ cd" => {},
                "$ cd .." => {curr_dir.pop();},
                _ => {
                    let (_, name) = line.rsplit_once(" ").unwrap();
                    curr_dir.push(name);
                }
            }
        }
        else if !line.starts_with("$ ls") {
            let (size_or_dir, name) = line.split_once(" ").unwrap();
            let mut path = curr_dir.clone();
            path.push(name);
            let size = match size_or_dir {
                "dir" => None,
                _ => Some(usize::from_str(size_or_dir).unwrap()),
            };
            let new_node;
            if let Some(size) = size {
                new_node = Node {
                    size: Cell::new(Some(size)),
                    is_directory: false,
                    children: vec![],
                    path: path.clone(),
                }
            }
            else {
                new_node = Node {
                        size: Cell::new(None),
                        is_directory: true,
                        children: vec![],
                        path: path.clone(),
                }
            }
            path_node_map.insert(path.clone(), new_node);
            path_node_map.get_mut(&curr_dir).unwrap().children.push(path);
        }
    }
    path_node_map
}


pub fn part_1(path_node_map: &HashMap<PathBuf, Node>) -> usize {
    let mut res = 0;
    for (_, node) in path_node_map.iter() {
        let size = node.set_total_size(&path_node_map);
        if size < 100000 && node.is_directory == true {
            res += size;
        }
    }
    res
}
pub fn part_2(path_node_map: &HashMap<PathBuf, Node>) -> usize {
    let mut sizes = vec![];
    for (_, node) in path_node_map.iter() {
        if node.is_directory {
            sizes.push(node.set_total_size(&path_node_map));
        }
    }
    let space_left = 70000000 - *sizes.iter().max().unwrap();
    let space_to_release = 30000000 - space_left;
    sizes.into_iter().filter(|size| size > &space_to_release).min().unwrap()
}