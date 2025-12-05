use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> u64 {
    let mut result = 0;
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i;
            break;
        }
    }
    let line_count = (input.len() + 1) / (line_length + 1);
    // with padding to not check for bounds
    let mut adjacency_table: Vec<Vec<usize>> = vec![vec![0; line_count + 2]; line_length + 2];
    for line_index in 0..line_count {
        for char_index in 0..line_length {
            if input[(line_index * (line_length + 1)) + char_index] == b'@' {
                adjacency_table[char_index][line_index] += 1;
                adjacency_table[char_index][line_index + 1] += 1;
                adjacency_table[char_index][line_index + 2] += 1;
                adjacency_table[char_index + 1][line_index] += 1;
                adjacency_table[char_index + 1][line_index + 2] += 1;
                adjacency_table[char_index + 2][line_index] += 1;
                adjacency_table[char_index + 2][line_index + 1] += 1;
                adjacency_table[char_index + 2][line_index + 2] += 1;
            }
        }
    }
    for line_index in 0..line_count {
        for char_index in 0..line_length {
            if input[line_index * (line_length + 1) + char_index] == b'@'
                && adjacency_table[char_index + 1][line_index + 1] < 4
            {
                result += 1;
            }
        }
    }
    return result;
}

pub fn solve_part1_input_mut(input: &[u8]) -> u64 {
    let mut result = 0;
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i;
            break;
        }
    }
    // with padding to not check for bounds
    let mut adjacency_list = vec![0; input.len() + (2 * line_length) + 2];
    for index in 0..input.len() {
        if input[index] == b'@' {
            adjacency_list[index] += 1;
            adjacency_list[index + line_length] += 1;
            adjacency_list[index + (2 * line_length)] += 1;
            adjacency_list[index + 1] += 1;
            adjacency_list[index + 1 + (2 * line_length)] += 1;
            adjacency_list[index + 2] += 1;
            adjacency_list[index + 2 + line_length] += 1;
            adjacency_list[index + 2 + (2 * line_length)] += 1;
        }
    }
    for index in 0..input.len() {
        if input[index] == b'@' && adjacency_list[index + line_length + 1] < 4 {
            result += 1;
        }
    }
    return result;
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut result = 0;
    let mut line_length = 0;
    for i in 0..input.len() {
        if input[i] == b'\n' {
            line_length = i;
            break;
        }
    }
    let line_count = (input.len() + 1) / (line_length + 1);
    // with padding to not check for bounds
    let mut adjacency_table: Vec<Vec<usize>> = vec![vec![0; line_count + 2]; line_length + 2];
    for line_index in 0..line_count {
        for char_index in 0..line_length {
            if input[(line_index * (line_length + 1)) + char_index] == b'@' {
                adjacency_table[char_index][line_index] += 1;
                adjacency_table[char_index][line_index + 1] += 1;
                adjacency_table[char_index][line_index + 2] += 1;
                adjacency_table[char_index + 1][line_index] += 1;
                adjacency_table[char_index + 1][line_index + 2] += 1;
                adjacency_table[char_index + 2][line_index] += 1;
                adjacency_table[char_index + 2][line_index + 1] += 1;
                adjacency_table[char_index + 2][line_index + 2] += 1;
            }
        }
    }
    let mut input_clone = input.to_vec();
    let mut removed_roll = true;
    while removed_roll {
        removed_roll = false;
        for line_index in 0..line_count {
            for char_index in 0..line_length {
                if input_clone[line_index * (line_length + 1) + char_index] == b'@'
                    && adjacency_table[char_index + 1][line_index + 1] < 4
                {
                    adjacency_table[char_index][line_index] -= 1;
                    adjacency_table[char_index][line_index + 1] -= 1;
                    adjacency_table[char_index][line_index + 2] -= 1;
                    adjacency_table[char_index + 1][line_index] -= 1;
                    adjacency_table[char_index + 1][line_index + 2] -= 1;
                    adjacency_table[char_index + 2][line_index] -= 1;
                    adjacency_table[char_index + 2][line_index + 1] -= 1;
                    adjacency_table[char_index + 2][line_index + 2] -= 1;
                    input_clone[line_index * (line_length + 1) + char_index] = b'x';
                    result += 1;
                    removed_roll = true;
                }
            }
        }
    }
    return result;
}

test_day!(
    4,
    b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    13,
    43
);
