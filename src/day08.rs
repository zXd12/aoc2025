use std::collections::BinaryHeap;

use crate::utils::test_day;

/*
as per the problem formulation, the following conditions are required to avoid undefined behaviour:
    - there are at least 3 different networks after applying the number_of_connections first connections
    - there are no tied euclidian distances for the last connection before the cutoff
    - the number of connections to keep is different between the exemple value and standrad inputs.
        the number of junction boxes is used to discriminate between the 2 cases, with an arbitrary cutoff at 100
added assumptions:
    - the input ends with a \n
    - the euclidian distance computation can be done in isize bounds
    - every euclidian distance is unique
*/
pub fn solve_part1(input: &[u8]) -> u64 {
    let junction_box_positions = parse(input);
    // arbitrary cutoff to handle both the exemple and standard input
    let number_of_connections = if junction_box_positions.len() < 100 {
        10
    } else {
        1000
    };
    // compute the number_of_connections closest distances
    let mut closest_distances = BinaryHeap::with_capacity(number_of_connections + 1);
    for x in 0..junction_box_positions.len() {
        let box_x = junction_box_positions[x];
        for y in (x + 1)..junction_box_positions.len() {
            let box_y = junction_box_positions[y];
            let euclidian_distance = euclidian_distance(box_x, box_y);
            // this is always true for only the first number_of_connections elements
            // it would technicaly be possible to fill the BTree in 2 distinct steps
            // so the next 498500 iterations have one less condition to check
            // (in the case of the standard inputs which have 1000*999/2 total connections)
            // but I don't see a clean way to that
            if closest_distances.len() < number_of_connections {
                closest_distances.push((euclidian_distance, x, y));
            } else if closest_distances.peek().unwrap().0 > euclidian_distance {
                closest_distances.push((euclidian_distance, x, y));
                closest_distances.pop();
            }
        }
    }
    // compute the networks
    let mut networks = vec![0; junction_box_positions.len()]; // 0 = not part of a network
    let mut network_index = 1;
    // BinaryHeap.iter() does not keep the order of the elements, but we don't care about it at that point anyway
    for (_, x, y) in closest_distances.iter().rev() {
        match (networks[*x], networks[*y]) {
            (0, 0) => {
                // create a new network
                networks[*x] = network_index;
                networks[*y] = network_index;
                network_index += 1;
            }
            (0, network_y) => networks[*x] = network_y,
            (network_x, 0) => networks[*y] = network_x,
            (network_x, network_y) => {
                if network_x != network_y {
                    // merges the 2 networks
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
    // network 0 is for boxes without a network. it should be counted as multiple networks of size 1
    // we don't need to represent all of them in the list, as we only need the 3 biggest networks
    // the worst case is that there is no network at all, which can be handled by having 3 1-sized "networks"
    network_sizes[0] = 1;
    network_sizes.push(1);
    network_sizes.push(1);
    network_sizes.sort();
    return network_sizes[network_sizes.len() - 1]
        * network_sizes[network_sizes.len() - 2]
        * network_sizes[network_sizes.len() - 3];
}

/*
as per the problem formulation, the following conditions are required to avoid undefined behaviour:
    - there are no tied euclidian distances capable of closing the network
added assumptions:
    - the input ends with a \n
    - the euclidian distance computation can be done in isize bounds
    - every euclidian distance is unique
*/
pub fn solve_part2(input: &[u8]) -> u64 {
    let junction_box_positions = parse(input);
    // compute the number_of_connections closest distances
    let mut distances = vec![];
    for x in 0..junction_box_positions.len() {
        let box_x = junction_box_positions[x];
        for y in (x + 1)..junction_box_positions.len() {
            let box_y = junction_box_positions[y];
            distances.push((euclidian_distance(box_x, box_y), x, y));
        }
    }
    // :skull:
    // 95% of the runtime of the function is spent here
    // One way to be faster than this would be to build the networks during sorting, as soon as elements are in their final position
    // My input closes the network after less than 5000 connections, so the remaining 499000 ones do not technicaly need to be sorted
    // This would be way faster for inputs with an even distribution, which require ~ 2/n*​ln(n) edges for a complete graph
    // (according to chatgpt, taking purely random edges, or 3454 edges for 1000 nodes, which is not that far of my 4710)
    // But synthetic data with, for example, a junction box far from the others, forces the sorting of the entire array to get to it being the next closest one
    // And my implementation would of course be much slower than rust builtin unstable sort.
    distances.sort_unstable_by_key(|&(dist, _, _)| dist);
    // compute the networks
    let mut networks = vec![0; junction_box_positions.len()];
    let mut network_sizes = vec![0];
    let mut grown_network = 0; // the network that was increased in size from the operation
    let mut distance_index = 0;
    for (_, x, y) in distances {
        distance_index += 1;
        match (networks[x], networks[y]) {
            (0, 0) => {
                // new network of size 2
                grown_network = network_sizes.len();
                networks[x] = grown_network;
                networks[y] = grown_network;
                network_sizes.push(2);
            }
            (0, network_y) => {
                grown_network = network_y;
                networks[x] = grown_network;
                network_sizes[grown_network] += 1;
            }
            (network_x, 0) => {
                grown_network = network_x;
                networks[y] = grown_network;
                network_sizes[grown_network] += 1;
            }
            (network_x, network_y) => {
                if network_x != network_y {
                    grown_network = network_x;
                    // I feel like there should be something faster than iterating over the entire list
                    // but after thinking about it for a while I couldn't find anything
                    // the code goes through this loop 295 times with my input, so 295 000 total comparison
                    // which is a lot, but not that much compared to the n log(n) sort of the 500K elements distances array
                    for i in 0..networks.len() {
                        if networks[i] == network_y {
                            networks[i] = grown_network;
                        }
                    }
                    network_sizes[grown_network] += network_sizes[network_y];
                }
            }
        }
        if network_sizes[grown_network] >= networks.len() {
            println!("distance n°{distance_index}");
            return (junction_box_positions[x][0] * junction_box_positions[y][0]) as u64;
        }
    }
    unreachable!()
}

fn parse(input: &[u8]) -> Vec<[isize; 3]> {
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
    junction_box_positions
}

fn euclidian_distance(a: [isize; 3], b: [isize; 3]) -> isize {
    (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2)
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
