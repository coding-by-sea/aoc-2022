use std::collections::VecDeque;

pub fn part_1(lines: &Vec<String>) -> usize {
    get_res(&lines[0], 4)
}

pub fn part_2(lines: &Vec<String>) -> usize {
    get_res(&lines[0], 14)
}

fn get_res(string: &String, window_length: usize) -> usize {
    let mut tracker = [0; 26];
    let mut window = VecDeque::with_capacity(window_length);
    let num_a = u8::try_from('a').unwrap();
    for (i, char) in string.as_bytes().iter().enumerate() {
        let num_char = *char - num_a;
        let mut num_unique = 0;
        window.push_back(num_char);
        tracker[num_char as usize] += 1;
        if window.len() > window_length {
            let index = window.pop_front().unwrap() as usize;
            tracker[index] -= 1;
        }
        if window.len() == window_length {
            for count in tracker {
                if count == 1 {
                    num_unique += 1;
                }
            }
            if num_unique == window_length {
                return i + 1;
            }
        }
    }
    return 0;
}