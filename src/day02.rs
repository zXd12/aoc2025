use std::u32;

use crate::utils::test_day;

// ------- part 1 -------
struct Duplicate {
    half: u32,
    digits: u32,
}

impl Duplicate {
    fn value(&self) -> u64 {
        (self.half as u64) * (10u64.pow(self.digits) + 1)
    }

    fn next(&self) -> Duplicate {
        if self.half == 10u32.pow(self.digits) - 1 {
            Duplicate {
                half: 10u32.pow(self.digits + 1),
                digits: self.digits + 2,
            }
        } else {
            Duplicate {
                half: self.half + 1,
                digits: self.digits,
            }
        }
    }
}

fn next_duplicate_from_any_number(number: u64, digits: u32) -> Duplicate {
    // returns one half of the next duplicate number, and the number of digits in the half
    if digits % 2 == 1 {
        // odd digit count
        Duplicate {
            half: 10u32.pow(digits / 2),
            digits: (digits + 1) / 2,
        }
    } else {
        // even digit count
        let top = (number / (10u64.pow(digits / 2))) as u32;
        let bottom = (number % (10u64.pow(digits / 2))) as u32;
        Duplicate {
            half: top + if top < bottom { 1 } else { 0 },
            digits: digits / 2,
        }
    }
}

pub fn solve_part1(input: &[u8]) -> u64 {
    let mut res: u64 = 0;
    let mut index = 0;
    while index < input.len() {
        let mut first_number = 0;
        let mut first_number_digits = 0;
        while input[index] != b'-' {
            first_number_digits += 1;
            first_number *= 10;
            first_number += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;
        let mut second_number = 0;
        while index < input.len() && input[index] != b',' && input[index] != b'\n' {
            second_number *= 10;
            second_number += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;

        let mut duplicate = next_duplicate_from_any_number(first_number, first_number_digits);
        while duplicate.value() <= second_number {
            res += duplicate.value() as u64;
            duplicate = duplicate.next();
        }
    }
    return res;
}

// ------- part 2 -------
#[derive(Debug, Clone, Copy)]
struct Multiplicate {
    subpart: u64,
    digits: u32,
    repeats: u32,
}

impl Multiplicate {
    fn value(&self) -> u64 {
        let mut res = 0;
        for _ in 0..self.repeats {
            res *= 10u64.pow(self.digits);
            res += self.subpart as u64;
        }
        res
    }

    fn next(next_multiplicates_by_digit_count: &mut [Option<Multiplicate>; 22]) -> Multiplicate {
        // todo: handle single digit numbers
        let mut next_multiplicate = Multiplicate {
            subpart: u64::MAX - 1,
            digits: 19,
            repeats: 1,
        };
        let mut digit_count = 1;
        for potential_multiplicate_digit_count in 1..next_multiplicates_by_digit_count.len() {
            if let Some(multiplicate) =
                &next_multiplicates_by_digit_count[potential_multiplicate_digit_count]
            {
                if multiplicate.value() < next_multiplicate.value() {
                    next_multiplicate = multiplicate.clone();
                    digit_count = potential_multiplicate_digit_count;
                }
            }
        }
        let mut new_multiplicate_option = next_multiplicate.next_with_same_number_of_digits();
        if let Some(new_multiplicate) = new_multiplicate_option {
            for digits in 0..new_multiplicate.digits as usize {
                if let Some(multiplicate) = next_multiplicates_by_digit_count[digits].clone() {
                    if new_multiplicate.value() == multiplicate.value() {
                        new_multiplicate_option =
                            new_multiplicate.next_with_same_number_of_digits();
                        break;
                    }
                }
            }
        }
        next_multiplicates_by_digit_count[digit_count] = new_multiplicate_option;

        return next_multiplicate;
    }

    fn next_with_same_number_of_digits(&self) -> Option<Multiplicate> {
        if self.subpart == 10u64.pow(self.digits) - 1 {
            // all nines
            None
        } else {
            Some(Multiplicate {
                subpart: self.subpart + 1,
                digits: self.digits,
                repeats: self.repeats,
            })
        }
    }
}

fn next_multiplicate_from_any_number(
    number: u64,
    digits: u32,
    digit_per_subpart: u32,
) -> Option<Multiplicate> {
    // returns one half of the next duplicate number, and the number of digits in the half
    if digits % digit_per_subpart != 0 {
        None
    } else {
        let subpart_count = digits / digit_per_subpart;
        if subpart_count == 1 {
            return None;
        }
        let mut first_subpart =
            number / (10u64.pow((digits / subpart_count) * (subpart_count - 1)));

        for subpart_index in 2..=subpart_count {
            let subpart_value = number
                / (10u64.pow((digits / subpart_count) * (subpart_count - subpart_index)))
                % (10u64.pow(digits / subpart_count));
            if subpart_value < first_subpart {
                break;
            } else if subpart_value > first_subpart {
                first_subpart += 1;
                break;
            }
        }

        Some(Multiplicate {
            subpart: first_subpart as u64,
            digits: digit_per_subpart,
            repeats: subpart_count,
        })
    }
}

fn build_next_multiplicates_by_digit_count_table(
    number: u64,
    digits: u32,
) -> [Option<Multiplicate>; 22] {
    let mut res: [Option<Multiplicate>; 22] = [None; 22];
    for digit_count in 1..digits {
        res[digit_count as usize] = next_multiplicate_from_any_number(number, digits, digit_count);
        if let Some(new_multiplicate) = res[digit_count as usize] {
            for digits in 0..digit_count as usize {
                if let Some(multiplicate) = res[digits].clone() {
                    if new_multiplicate.value() == multiplicate.value() {
                        res[digit_count as usize] =
                            new_multiplicate.next_with_same_number_of_digits();
                        break;
                    }
                }
            }
        }
    }
    res
}

pub fn solve_part2(input: &[u8]) -> u64 {
    let mut res: u64 = 0;
    let mut index = 0;
    while index < input.len() {
        let mut first_number = 0;
        let mut first_number_digits = 0;
        while input[index] != b'-' {
            first_number_digits += 1;
            first_number *= 10;
            first_number += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;
        let mut second_number = 0;
        let mut second_number_digits = 0;
        while index < input.len() && input[index] != b',' && input[index] != b'\n' {
            second_number_digits += 1;
            second_number *= 10;
            second_number += (input[index] - b'0') as u64;
            index += 1;
        }
        index += 1;

        let mut tables = vec![build_next_multiplicates_by_digit_count_table(
            first_number,
            first_number_digits,
        )];
        if first_number_digits < second_number_digits {
            for digits in first_number_digits..second_number_digits {
                tables.push(build_next_multiplicates_by_digit_count_table(
                    10u64.pow(digits),
                    digits + 1,
                ));
            }
        }
        for mut table in tables {
            let mut multiplicate = Multiplicate::next(&mut table);
            while multiplicate.value() <= second_number {
                res += multiplicate.value();
                multiplicate = Multiplicate::next(&mut table);
            }
        }
    }
    return res;
}

test_day!(
    2,
    b"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
    1227775554,
    4174379265
);
