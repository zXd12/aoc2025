use std::collections::BTreeSet;

use crate::utils::test_day;

pub fn solve_part1(input: &[u8]) -> u64 {
    let mut junction_box_positions = vec![];
    let mut acc = 0;
    let mut junction_box_position = [0; 3];
    let mut coordinate_index = 0;
    for byte in input {
        match byte {
            b'0'..=b'9' => {
                acc *= 10;
                acc += (byte - b'0') as isize;
            }
            b',' => {
                junction_box_position[coordinate_index] = acc;
                coordinate_index += 1;
                acc = 0;
            }
            b'\n' => {
                junction_box_position[coordinate_index] = acc;
                coordinate_index = 0;
                acc = 0;
                junction_box_positions.push(junction_box_position);
                junction_box_position = [0; 3];
            }
            _ => panic!("Unexpected char {}", *byte as char),
        }
    }
    // arbitrary cutoff to handle both the exemple and standard input
    let number_of_connections = if junction_box_positions.len() < 100 {
        10
    } else {
        1000
    };
    // compute the number_of_connections closest distances
    let mut closest_distances = BTreeSet::new();
    for x in 0..junction_box_positions.len() {
        let box_x = junction_box_positions[x];
        for y in (x + 1)..junction_box_positions.len() {
            let box_y = junction_box_positions[y];
            let euclidian_distance = (box_x[0] - box_y[0]).pow(2)
                + (box_x[1] - box_y[1]).pow(2)
                + (box_x[2] - box_y[2]).pow(2);
            if closest_distances.len() < number_of_connections {
                closest_distances.insert((euclidian_distance, x, y));
            } else if closest_distances.last().unwrap().0 > euclidian_distance {
                closest_distances.insert((euclidian_distance, x, y));
                closest_distances.pop_last();
            }
        }
    }
    // compute the networks
    let mut networks = vec![0; junction_box_positions.len()];
    let mut network_index = 1;
    for (_, x, y) in closest_distances {
        match (networks[x], networks[y]) {
            (0, 0) => {
                networks[x] = network_index;
                networks[y] = network_index;
                network_index += 1;
            }
            (0, network_y) => networks[x] = network_y,
            (network_x, 0) => networks[y] = network_x,
            (network_x, network_y) => {
                if network_x != network_y {
                    // sooo inefficient
                    for i in 0..networks.len() {
                        if networks[i] == network_y {
                            networks[i] = network_x;
                        }
                    }
                }
            }
        }
    }
    // count the number of junction box in each network
    let mut network_sizes = vec![0; network_index];
    for network in networks {
        network_sizes[network] += 1;
    }
    network_sizes[0] = 1;
    network_sizes.sort();
    return network_sizes[network_sizes.len() - 1]
        * network_sizes[network_sizes.len() - 2]
        * network_sizes[network_sizes.len() - 3];
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut junction_box_positions = vec![];
    let mut acc = 0;
    let mut junction_box_position = [0; 3];
    let mut coordinate_index = 0;
    for byte in input {
        match byte {
            b'0'..=b'9' => {
                acc *= 10;
                acc += (byte - b'0') as isize;
            }
            b',' => {
                junction_box_position[coordinate_index] = acc;
                coordinate_index += 1;
                acc = 0;
            }
            b'\n' => {
                junction_box_position[coordinate_index] = acc;
                coordinate_index = 0;
                acc = 0;
                junction_box_positions.push(junction_box_position);
                junction_box_position = [0; 3];
            }
            _ => panic!("Unexpected char {}", *byte as char),
        }
    }
    // compute the number_of_connections closest distances
    let mut distances = vec![];
    for x in 0..junction_box_positions.len() {
        let box_x = junction_box_positions[x];
        for y in (x + 1)..junction_box_positions.len() {
            let box_y = junction_box_positions[y];
            let euclidian_distance = (box_x[0] - box_y[0]).pow(2)
                + (box_x[1] - box_y[1]).pow(2)
                + (box_x[2] - box_y[2]).pow(2);
            distances.push((euclidian_distance, x, y));
        }
    }
    distances.sort_unstable_by_key(|&(dist, _, _)| dist);
    // compute the networks
    let mut networks = vec![0; junction_box_positions.len()];
    let mut network_sizes = vec![0; junction_box_positions.len()];
    let mut network_index = 1;
    let mut increased_network = 0;
    for (_, x, y) in distances {
        match (networks[x], networks[y]) {
            (0, 0) => {
                increased_network = network_index;
                networks[x] = increased_network;
                networks[y] = increased_network;
                network_sizes[increased_network] += 2;
                network_index += 1;
            }
            (0, network_y) => {
                increased_network = network_y;
                networks[x] = increased_network;
                network_sizes[increased_network] += 1;
            }
            (network_x, 0) => {
                increased_network = network_x;
                networks[y] = increased_network;
                network_sizes[increased_network] += 1;
            }
            (network_x, network_y) => {
                if network_x != network_y {
                    increased_network = network_x;
                    for i in 0..networks.len() {
                        if networks[i] == network_y {
                            networks[i] = increased_network;
                            network_sizes[increased_network] += 1;
                        }
                    }
                }
            }
        }
        if network_sizes[increased_network] >= networks.len() {
            return (junction_box_positions[x][0] * junction_box_positions[y][0]) as u64;
        }
    }
    return 0;
}

test_day!(
    8,
    b"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
    40,
    25272
);
