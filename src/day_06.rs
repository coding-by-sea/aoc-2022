pub fn part_1(lines: &Vec<String>) -> usize {
    get_res(&lines[0], 4)
}

pub fn part_2(lines: &Vec<String>) -> usize {
    get_res(&lines[0], 14)
}

fn get_res(string: &String, window_length: usize) -> usize {
    let mut tracker = [0; 26];
    let mut last_pop_pos: Option<usize> = None;
    let num_a = u8::try_from('a').unwrap();
    let bytes: Vec<_> = string.as_bytes().iter().collect();
    for (i, &char) in bytes.iter().enumerate() {
        let mut num_unique = 0;
        tracker[(*char - num_a) as usize] += 1;
        let mut length;
        match last_pop_pos {
            Some(last_pop_pos) => length = i - last_pop_pos,
            None => length = i + 1
        }
        if length > window_length {
            let index = i - window_length;
            tracker[(*(bytes[index]) - num_a) as usize] -= 1;
            last_pop_pos = Some(index);
            length -= 1;
        }
        if length == window_length {
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