use std::cmp::max;

use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> u64 {
    let mut result = 0;
    let mut ranges = vec![];
    let mut index = if input[0] == b'\n' { 1 } else { 0 };
    while input[index] != b'\n' {
        let mut range_start = 0;
        while input[index] != b'-' {
            range_start *= 10;
            range_start += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;
        let mut range_end = 0;
        while input[index] != b'\n' {
            range_end *= 10;
            range_end += (input[index] - b'0') as u64;
            index += 1;
        }
        ranges.push((range_start, range_end));
        index += 1;
    }
    index += 1;
    ranges.sort_by(|a, b| (a.0).cmp(&b.0));
    let mut ids = vec![];
    while index < input.len() {
        let mut id = 0;
        while index < input.len() && input[index] != b'\n' {
            id *= 10;
            id += (input[index] - b'0') as u64;
            index += 1;
        }
        ids.push(id);
        index += 1;
    }
    ids.sort();
    let mut range_index = 0;
    'outer_loop: for id in ids {
        while ranges[range_index].1 < id {
            range_index += 1;
            if range_index >= ranges.len() {
                break 'outer_loop;
            }
        }
        if id >= ranges[range_index].0 {
            result += 1;
        }
    }
    return result;
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut result = 0;
    let mut ranges = vec![];
    let mut index = if input[0] == b'\n' { 1 } else { 0 };
    while input[index] != b'\n' {
        let mut range_start = 0;
        while input[index] != b'-' {
            range_start *= 10;
            range_start += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;
        let mut range_end = 0;
        while input[index] != b'\n' {
            range_end *= 10;
            range_end += (input[index] - b'0') as u64;
            index += 1;
        }
        ranges.push((range_start, range_end));
        index += 1;
    }
    ranges.sort_by(|a, b| (a.0).cmp(&b.0));
    // early parsing termination, we can ignore the 2nd half of the input
    let mut current_range = (1, 0);
    for new_range in ranges {
        if new_range.0 <= current_range.1 {
            current_range.1 = max(current_range.1, new_range.1);
        } else {
            result += current_range.1 + 1 - current_range.0;
            current_range = new_range;
        }
    }
    result += current_range.1 + 1 - current_range.0;
    return result;
}

test_day!(
    5,
    b"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3,
    14
);
