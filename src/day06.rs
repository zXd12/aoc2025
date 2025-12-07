use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> u64 {
    // start from the end to get the operators first
    let mut index = input.len() - 1;
    let mut is_mult = vec![];
    loop {
        index -= 1;
        match input[index] {
            b' ' => continue,
            b'+' => is_mult.push(false),
            b'*' => is_mult.push(true),
            b'\n' => break,
            _ => panic!("unexpected char {} in last line", input[index] as char),
        }
    }
    let mut collumns = Vec::with_capacity(is_mult.len());
    let mut acc = 0;
    let mut digit_count = 0;
    loop {
        index -= 1;
        match input[index] {
            b' ' => {
                if digit_count != 0 {
                    collumns.push(acc);
                    acc = 0;
                    digit_count = 0;
                }
            }
            b'\n' => {
                if digit_count != 0 {
                    collumns.push(acc);
                    acc = 0;
                    digit_count = 0;
                }
                break;
            }
            b'0'..=b'9' => {
                acc += (input[index] - b'0') as u64 * 10_u64.pow(digit_count);
                digit_count += 1;
            }
            _ => panic!("unexpected char {}", input[index] as char),
        }
    }
    let mut collumn_index = 0;
    while index > 0 {
        index -= 1;
        match input[index] {
            b' ' => {
                if digit_count != 0 {
                    if is_mult[collumn_index] {
                        collumns[collumn_index] *= acc;
                    } else {
                        collumns[collumn_index] += acc;
                    }
                    acc = 0;
                    digit_count = 0;
                    collumn_index += 1;
                }
            }
            b'\n' => {
                if digit_count != 0 {
                    if is_mult[collumn_index] {
                        collumns[collumn_index] *= acc;
                    } else {
                        collumns[collumn_index] += acc;
                    }
                    acc = 0;
                    digit_count = 0;
                }
                collumn_index = 0;
            }
            b'0'..=b'9' => {
                acc += (input[index] - b'0') as u64 * 10_u64.pow(digit_count);
                digit_count += 1;
            }
            _ => panic!("unexpected char {}", input[index] as char),
        }
    }
    if digit_count != 0 {
        if is_mult[collumn_index] {
            collumns[collumn_index] *= acc;
        } else {
            collumns[collumn_index] += acc;
        }
    }
    return collumns.iter().sum();
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
    let line_count = input.len() / line_length;
    let mut last_operator_row_index = line_length + 1;
    for row_index in (0..line_length).rev() {
        let operator_index = ((line_count - 1) * (line_length + 1)) + row_index;
        if input[operator_index] != b' ' {
            let is_mult = input[operator_index] == b'*';
            let mut problem_result = if is_mult { 1 } else { 0 };
            for row in row_index..last_operator_row_index - 1 {
                let mut acc = 0;
                for line in 0..line_count - 1 {
                    let byte = input[(line * (line_length + 1)) + row];
                    if byte == b' ' {
                        if acc != 0 {
                            break;
                        }
                    } else {
                        acc *= 10;
                        acc += (byte - b'0') as u64
                    }
                }
                problem_result = if is_mult {
                    problem_result * acc
                } else {
                    problem_result + acc
                }
            }
            result += problem_result;
            last_operator_row_index = row_index;
        }
    }
    return result;
}

test_day!(
    6,
    b"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
    4277556,
    3263827
);
