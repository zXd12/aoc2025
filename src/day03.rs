use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> i32 {
    let mut result = 0;
    let mut index = 0;
    while index < input.len() {
        let mut right = 0;
        let mut left = 0;
        while index + 1 < input.len() && input[index + 1] != b'\n' {
            let digit = input[index] - b'0';
            if digit > right {
                right = digit;
                left = 0;
            } else if digit > left {
                left = digit;
            }
            index += 1;
        }
        if input[index] - b'0' > left {
            left = input[index] - b'0';
        }
        result += (right * 10 + left) as i32;
        index += 2;
    }
    return result;
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i;
            break;
        }
    }
    let line_count = input.len() / (line_length + 1);
    let mut result = 0;
    for line_index in 0..line_count {
        let line_input_index = (line_length + 1) * line_index;
        let mut cutoff_index = 0;
        for joltage_index in (0..12).rev() {
            let (mut max_value, mut max_index) = (0, 0);
            for i in cutoff_index..line_length - joltage_index {
                if input[line_input_index + i] > max_value {
                    (max_value, max_index) = (input[line_input_index + i], i);
                    if max_value == 9 {
                        break;
                    }
                }
            }
            cutoff_index = max_index + 1;
            result += (max_value - b'0') as u64 * 10u64.pow(joltage_index as u32);
        }
    }
    return result;
}

test_day!(
    3,
    b"987654321111111
811111111111119
234234234234278
818181911112111
",
    357,
    3121910778619
);
