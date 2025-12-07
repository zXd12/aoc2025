use crate::utils::test_day;

// using bitwise operators on bigints would probably be faster, but I'm not using external libraries
pub fn solve_part1(input: &[u8]) -> u64 {
    let mut result: u64 = 0;
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i + 1;
            break;
        }
    }
    let line_count = input.len() / line_length / 2;
    let mut tachion_vec = vec![false; line_length * line_count];
    tachion_vec[(line_length / 2) - 1] = true;
    for line_index in 1..line_count {
        let line_first_byte_index = (line_index * (line_length - 1)) + (line_length / 2);
        for row_index in 0..line_index {
            let byte_index = line_first_byte_index + (2 * row_index);
            if tachion_vec[byte_index - line_length]
                && input[byte_index + (line_index * line_length)] == b'^'
            {
                result += 1;
                tachion_vec[byte_index - 1] = true;
                tachion_vec[byte_index + 1] = true;
            }
        }
    }
    return result;
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i + 1;
            break;
        }
    }
    let line_count = input.len() / line_length / 2;
    let mut tachion_vec = vec![0; line_length * line_count];
    tachion_vec[(line_length / 2) - 1] = 1u64;
    for line_index in 1..line_count {
        let line_first_byte_index = (line_index * (line_length - 1)) + (line_length / 2);
        for row_index in 0..line_index {
            let byte_index = line_first_byte_index + (2 * row_index);
            if input[byte_index + (line_index * line_length)] == b'^' {
                tachion_vec[byte_index - 1] += tachion_vec[byte_index - line_length];
                tachion_vec[byte_index + 1] += tachion_vec[byte_index - line_length];
            } else if line_index < line_count - 1 {
                tachion_vec[byte_index + line_length] = tachion_vec[byte_index - line_length];
            } else {
                tachion_vec[byte_index] = tachion_vec[byte_index - line_length];
            }
        }
    }
    let mut result = 0;
    for i in 0..line_length {
        result += tachion_vec[tachion_vec.len() - i - 1]
    }
    return result;
}

test_day!(
    7,
    b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
    21,
    40
);
