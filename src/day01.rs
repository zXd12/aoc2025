use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> i32 {
    let mut result = 0;
    let mut position: i32 = 50;
    let mut index = 0;
    while index < input.len() {
        // -- parsing --
        let is_left = input[index] == b'L';
        let mut delta = 0;
        index += 1;
        // index < input.len() may be removed if the file ends with a \n
        while index < input.len() && input[index] != b'\n' {
            delta *= 10;
            delta += (input[index] - b'0') as i32;
            index += 1;
        }
        index += 1;

        // -- handle result --
        position += if is_left { -delta } else { delta };
        // we don't even have to get `position` back between 0 and 99
        // we only care if its divisible by 100
        result += (position % 100 == 0) as i32;
    }
    return result.into();
}

pub fn solve_part2(input: &[u8]) -> i32 {
    let mut result = 0;
    let mut position: i32 = 50;
    let mut index = 0;
    while index < input.len() {
        // -- parsing --
        let is_left = input[index] == b'L';
        let mut delta = 0;
        index += 1;
        // index < input.len() may be removed if the file ends with a \n
        while index < input.len() && input[index] != b'\n' {
            delta *= 10;
            delta += (input[index] - b'0') as i32;
            index += 1;
        }
        index += 1;

        // -- handle result --
        if is_left {
            delta = -delta
        };
        if position + delta <= 0 && position != 0 {
            result += 1;
        }
        position += delta;
        result += (position / 100).abs();
        // the following step could be done in 2 instructions instead of 3
        // position += 1_000_000_000; fine in our case where nothing will come close to the i32 bounds
        // position %= 100;
        // but for some black compiler magic reason this way is faster
        position %= 100;
        position += 100;
        position %= 100;
    }
    return result;
}

pub fn solve_both(input: &[u8]) -> (i32, i32) {
    // solves both parts with a single parse
    // not sure if I'll do this for every puzzle
    let mut result1 = 0;
    let mut result2 = 0;
    let mut position: i32 = 50;
    let mut index = 0;
    while index < input.len() {
        // -- parsing --
        let is_left = input[index] == b'L';
        let mut delta = 0;
        index += 1;
        while index < input.len() && input[index] != b'\n' {
            delta *= 10;
            delta += (input[index] - b'0') as i32;
            index += 1;
        }
        index += 1;

        // -- handle result --
        if is_left {
            delta = -delta
        };
        if position + delta <= 0 && position != 0 {
            result2 += 1;
        }
        position += delta;
        result2 += (position / 100).abs();
        position %= 100;
        position += 100;
        position %= 100;

        // only difference with the part2 solution
        result1 += (position % 100 == 0) as i32;
    }
    return (result1, result2);
}

test_day!(
    1,
    b"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
    3,
    6
);
