use std::arch::x86_64::_mm_test_all_ones;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::str::FromStr;
use anyhow::{anyhow, Result, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct Monkey {
    num: usize,
    starting_items: RefCell<VecDeque<usize>>,
    operation: String,
    operator: usize,
    test_divisor: usize,
    destination_if_true: usize,
    destination_if_false: usize,
    num_inspection: Cell<usize>,
}

impl Monkey {
    fn from_block(lines: &[String]) -> Self {
        let mut lines_iter = lines.iter();
        // parsing num
        let (_, monkey_num) = lines_iter.next().unwrap().split_once(" ").unwrap();
        let num = usize::from_str(&monkey_num[0..(monkey_num.len() - 1)]).unwrap();
        // parsing starting items
        let mut starting_items = VecDeque::new();
        let (_, mut items_str) = lines_iter.next().unwrap().split_once(":").unwrap();
        let items_str = items_str.trim();
        for item in items_str.split(", ") {
            starting_items.push_back(usize::from_str(item).unwrap());
        }
        // parsing operation and operator
        let operator;
        let operation;
        let (_, mut statement) = lines_iter.next().unwrap().split_once(":").unwrap();
        let (_, mut expression) = statement.split_once(" = ").unwrap();
        if expression == "old * old" {
            operation = "^".to_string();
            operator = 2;
        } else {
            let expression_elements: Vec<&str> = expression.split(" ").collect();
            operation = expression_elements[1].to_string();
            operator = usize::from_str(expression_elements[2]).unwrap();
        }
        // parsing divisor
        let (_, divisor_str) = lines_iter.next().unwrap().rsplit_once(" ").unwrap();
        let test_divisor = usize::from_str(divisor_str).unwrap();
        // parsing destination_if_true
        let (_, destination) = lines_iter.next().unwrap().rsplit_once(" ").unwrap();
        let destination_if_true = usize::from_str(destination).unwrap();
        // parsing destination_if_false
        let (_, destination) = lines_iter.next().unwrap().rsplit_once(" ").unwrap();
        let destination_if_false = usize::from_str(destination).unwrap();
        Monkey {
            num,
            starting_items: RefCell::new(starting_items),
            operation,
            operator,
            test_divisor,
            destination_if_true,
            destination_if_false,
            num_inspection: Cell::new(0),
        }
    }
    fn proceed(&self, monkeys: &Vec<Monkey>, reducer: impl Fn(usize) -> usize) {
        while let Some(mut item) = self.starting_items.borrow_mut().pop_front() {
            self.num_inspection.replace(self.num_inspection.get() + 1);
            item = match self.operation.as_str() {
                "*" => item * self.operator,
                "^" => item.pow(self.operator as u32),
                "+" => item + self.operator,
                _ => panic!("unknown operator"),
            };
            item = reducer(item);
            if item % self.test_divisor == 0 {
                monkeys[self.destination_if_true].starting_items.borrow_mut().push_back(item);
            }
            else {
                monkeys[self.destination_if_false].starting_items.borrow_mut().push_back(item);
                }
            }
        }
    }


pub fn parsing(lines: &Vec<String>) -> Vec<Monkey> {
    let mut res = vec![];
    for block in lines.split(|x| x.as_str() == "") {
        res.push(Monkey::from_block(block))
    }
    res
}


pub fn part_1(monkeys: & Vec<Monkey>) -> usize{
    for _ in 0..20 {
        for monkey in monkeys {
            monkey.proceed(&monkeys, |x| x / 3);
        }
    }
    let mut nums: Vec<_> = monkeys.iter().map(|monkey| monkey.num_inspection.get()).collect();
    nums.sort();
    assert!(nums.len() > 1);
    nums[nums.len()-1] * nums[nums.len()-2]
}

pub fn part_2(monkeys: & Vec<Monkey>) -> usize {
    let scaling_factor: usize = monkeys.iter().map(|x| x.test_divisor).product();
    for _ in 0..10000 {
        for monkey in monkeys {
            monkey.proceed(&monkeys, |x| x % scaling_factor);
        }
    }
    let mut nums: Vec<_> = monkeys.iter().map(|monkey| monkey.num_inspection.get()).collect();
    nums.sort();
    assert!(nums.len() > 1);
    nums[nums.len()-1] * nums[nums.len()-2]
}
