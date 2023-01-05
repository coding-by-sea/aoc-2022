use std::str::FromStr;

pub fn parsing(lines: &Vec<String>) -> Vec<i32> {
    lines.iter().map(|x| i32::from_str(x).unwrap()).collect()
}

pub fn part_1(nums: &Vec<i32>) -> i32 {
    let cycle = nums.len() as i32;
    let mut nums_positions= (0..cycle as usize).collect::<Vec<usize>>();
    let mut zero_original_pos = 0;
    for (i, num) in nums.iter().enumerate() {
        let pos = (nums_positions.iter().position(|x| *x == i).unwrap()) as i32;
        let step = (*num).rem_euclid(cycle-1);
        for i in 0..step {
            (nums_positions[(pos+i).rem_euclid(cycle) as usize], nums_positions[(pos+i+1).rem_euclid(cycle) as usize]) = (nums_positions[(pos+i+1).rem_euclid(cycle) as usize], nums_positions[(pos+i).rem_euclid(cycle) as usize]);
        }
        if *num == 0 {
            zero_original_pos = i;
        }
    }
    let zero_pos = (nums_positions.iter().position(|x| *x == zero_original_pos).unwrap()) as i32;
    nums[nums_positions[(zero_pos+1000).rem_euclid(cycle) as usize]] + nums[nums_positions[(zero_pos+2000).rem_euclid(cycle) as usize]] + nums[nums_positions[(zero_pos+3000).rem_euclid(cycle) as usize]]
}

pub fn part_2(nums: &Vec<i32>) -> i64 {
    let cycle = nums.len() as i32;
    let mut nums_positions= (0..cycle as usize).collect::<Vec<usize>>();
    let mut zero_original_pos = 0;
    for (i, num) in nums.iter().enumerate().cycle().take((cycle * 10) as usize) {
        let pos = (nums_positions.iter().position(|x| *x == i).unwrap()) as i32;
        let step = ((811589153 as i32).rem_euclid(cycle-1) * (*num).rem_euclid(cycle-1)).rem_euclid(cycle-1);
        for i in 0..step {
            (nums_positions[(pos+i).rem_euclid(cycle) as usize], nums_positions[(pos+i+1).rem_euclid(cycle) as usize]) = (nums_positions[(pos+i+1).rem_euclid(cycle) as usize], nums_positions[(pos+i).rem_euclid(cycle) as usize]);
        }
        if *num == 0 {
            zero_original_pos = i;
        }
    }
    // println!("{:?}", nums_positions);
    let zero_pos = (nums_positions.iter().position(|x| *x == zero_original_pos).unwrap()) as i32;
    (811589153 as i64) * (nums[nums_positions[(zero_pos+1000).rem_euclid(cycle) as usize]] + nums[nums_positions[(zero_pos+2000).rem_euclid(cycle) as usize]] + nums[nums_positions[(zero_pos+3000).rem_euclid(cycle) as usize]]) as i64
}
