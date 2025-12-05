use std::env;

pub fn get_input_for_day(_day: u8) -> String {
    // get input from the aoc website
    dotenv::dotenv().ok();
    let session_cookie = match env::var("COOKIE") {
        Ok(cookie) => cookie,
        Err(_) => panic!("no user cookie found. set it in a COOKIE env variable"),
    };
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(format!("https://adventofcode.com/2025/day/{}/input", _day))
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .unwrap();
    return response.text().unwrap();
}

macro_rules! test_day {
    ($day_number:expr, $sample_input:expr, $part1_sample_solution:expr, $part2_sample_solution:expr) => {
        extern crate test;

        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::utils::get_input_for_day;
            use test::Bencher;

            #[test]
            fn test_part1() {
                assert_eq!(solve_part1($sample_input), $part1_sample_solution);
            }

            #[test]
            fn test_part2() {
                assert_eq!(solve_part2($sample_input), $part2_sample_solution);
            }

            #[test]
            fn run_part1() {
                let input = &get_input_for_day($day_number);
                let input_bytes = input.as_bytes();
                println!("{}", solve_part1(input_bytes))
            }

            #[test]
            fn run_part2() {
                let input = &get_input_for_day($day_number);
                let input_bytes = input.as_bytes();
                println!("{}", solve_part2(input_bytes))
            }

            #[bench]
            fn bench_part1(b: &mut Bencher) {
                let input = &get_input_for_day($day_number);
                let input_bytes = input.as_bytes();
                b.iter(|| solve_part1(input_bytes));
            }

            #[bench]
            fn bench_part2(b: &mut test::Bencher) {
                let input = &get_input_for_day($day_number);
                let input_bytes = input.as_bytes();
                b.iter(|| solve_part2(input_bytes));
            }
        }
    };
}
pub(crate) use test_day;
