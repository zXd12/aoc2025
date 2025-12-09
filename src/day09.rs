use std::cmp::{max, min};

use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> u64 {
    let red_tile_positions = parse(input);
    let mut biggest_rectangle = rectangle_size(red_tile_positions[0], red_tile_positions[1]);
    for x in 0..red_tile_positions.len() {
        for y in x..red_tile_positions.len() {
            let rectangle_size = rectangle_size(red_tile_positions[x], red_tile_positions[y]);
            if rectangle_size > biggest_rectangle {
                biggest_rectangle = rectangle_size;
            }
        }
    }
    return biggest_rectangle as u64;
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut red_tile_positions = parse(input);

    // x shrink
    red_tile_positions.sort_unstable_by_key(|x| x[0]);
    let mut x_table = vec![red_tile_positions[0][0]];
    for position in red_tile_positions.iter_mut() {
        if position[0] != x_table[x_table.len() - 1] {
            x_table.push(position[0]);
        }
        position[0] = x_table.len() - 1;
    }
    // y shrink
    red_tile_positions.sort_unstable_by_key(|x| x[1]);
    let mut y_table = vec![red_tile_positions[0][1]];
    for position in red_tile_positions.iter_mut() {
        if position[1] != y_table[y_table.len() - 1] {
            y_table.push(position[1]);
        }
        position[1] = y_table.len() - 1;
    }

    // return to orinigal ordering (copying the array would be faster, but I'll do that later)
    red_tile_positions.sort_unstable_by_key(|x| x[2]);

    // excatly one every 2 point in standard inputs
    // this patern would only break with 3 (verticaly or horizontally) aligned points
    let mut is_next_vertical = vec![false; red_tile_positions.len()];
    for i in 0..red_tile_positions.len() - 1 {
        if red_tile_positions[i][1] != red_tile_positions[i + 1][1] {
            is_next_vertical[i] = true
        }
    }
    if red_tile_positions[red_tile_positions.len() - 1][1] != red_tile_positions[0][1] {
        is_next_vertical[red_tile_positions.len() - 1] = true
    }

    let mut vertical_matrix = vec![vec![false; y_table.len()]; x_table.len()];
    for i in 0..red_tile_positions.len() {
        let position = red_tile_positions[i];
        if is_next_vertical[position[2]] {
            let x = position[0];
            let from = min(
                position[1],
                red_tile_positions[i + 1 % red_tile_positions.len()][1],
            );
            let to = max(
                position[1],
                red_tile_positions[i + 1 % red_tile_positions.len()][1],
            );
            for y in from..to {
                vertical_matrix[y][x] = true;
            }
        }
    }

    for line in vertical_matrix.iter_mut() {
        let mut state = false;
        for square in line {
            if *square {
                state = !state;
            }
            *square = state;
        }
    }

    let mut best_rect = [[0; 2]; 2];
    let mut max_area = 0;
    for x_offset in 0..x_table.len() {
        for y_offset in 0..y_table.len() {
            if !vertical_matrix[y_offset][x_offset] {
                continue;
            }
            let mut max_y = y_table.len() - y_offset;
            for x_size in 0..x_table.len() - x_offset {
                if !vertical_matrix[y_offset][x_offset + x_size] {
                    break;
                }
                for y_size in 0..max_y {
                    if !vertical_matrix[y_offset + y_size][x_offset + x_size] {
                        max_y = y_size;
                        break;
                    }
                    let rect_size = (((x_table[x_offset] as isize)
                        - (x_table[x_offset + x_size] as isize)
                        + 1)
                    .abs()
                        * ((y_table[y_offset] as isize) - (y_table[y_offset + y_size] as isize)
                            + 1)
                        .abs()) as usize;
                    if rect_size > max_area {
                        max_area = rect_size;
                        best_rect = [[y_offset, x_offset], [y_offset + y_size, x_offset + x_size]]
                    };
                }
            }
        }
    }
    println!("{best_rect:?}");
    return max_area as u64;
}

fn parse(input: &[u8]) -> Vec<[usize; 3]> {
    // x, y, original index (to keep the ordering)
    let mut red_tile_positions = vec![];
    let mut acc = 0;
    let mut red_tile_position = [0; 3];
    let mut position_index = 0;
    for byte in input {
        match byte {
            b'0'..=b'9' => {
                acc *= 10;
                acc += (byte - b'0') as usize;
            }
            b',' => {
                red_tile_position[0] = acc;
                acc = 0;
            }
            b'\n' => {
                red_tile_position[1] = acc;
                acc = 0;
                red_tile_position[2] = position_index;
                position_index += 1;
                red_tile_positions.push(red_tile_position);
                red_tile_position = [0; 3];
            }
            _ => panic!("Unexpected char {}", *byte as char),
        }
    }
    red_tile_positions
}

fn rectangle_size(a: [usize; 3], b: [usize; 3]) -> usize {
    (((a[0] as isize) - (b[0] as isize) + 1).abs() * ((a[1] as isize) - (b[1] as isize) + 1).abs())
        as usize
}

test_day!(
    9,
    b"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
",
    50,
    24
);
