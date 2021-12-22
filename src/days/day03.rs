use super::{Day03, DaySolution};
use std::error::Error;

impl DaySolution for Day03 {
    fn part_1(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let (diagnostic_report, num_of_bits) = read_input(input_file);
        let (gamma_rate, epsilon_rate) =
            find_gamma_and_epsilon_rates(diagnostic_report, num_of_bits);
        Ok((gamma_rate * epsilon_rate).to_string())
    }

    fn part_2(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let (diagnostic_report, num_of_bits) = read_input(input_file);
        let (o2_generator_rating, co2_scrubber_rating) =
            find_o2_generator_and_co2_scrubber_ratings(diagnostic_report, num_of_bits);
        Ok((o2_generator_rating * co2_scrubber_rating).to_string())
    }
}

// Shared
use std::fs;
fn read_input(input_file: &String) -> (Vec<usize>, usize) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let parsed_input = contents
        .lines()
        .map(|b_str| usize::from_str_radix(b_str, 2).unwrap())
        .collect();
    let num_of_bits = contents.find('\n').unwrap();

    (parsed_input, num_of_bits)
}

fn get_bit_at_pos(num: usize, pos: usize) -> usize {
    let mask = 1 << pos;
    (num & mask) >> pos
}

fn find_most_common_bit_value_in_pos(diagnostic_report: &Vec<usize>, pos: usize) -> usize {
    find_most_common_bit_value_in_pos_with_tie_breaker(diagnostic_report, pos, 1)
}
fn find_most_common_bit_value_in_pos_with_tie_breaker(
    diagnostic_report: &Vec<usize>,
    pos: usize,
    tie_breaker: usize,
) -> usize {
    let mut num_zeroes = 0;
    let mut num_ones = 0;

    for d in diagnostic_report {
        if get_bit_at_pos(*d, pos) == 0 {
            num_zeroes += 1;
        } else {
            num_ones += 1
        }
    }

    if num_zeroes > num_ones {
        0
    } else if num_ones > num_zeroes {
        1
    } else {
        tie_breaker
    }
}

// Part 1
fn find_gamma_and_epsilon_rates(
    diagnostic_report: Vec<usize>,
    num_of_bits: usize,
) -> (usize, usize) {
    let mut gamma_rate: usize = 0;
    let mut epsilon_rate_mask = 0;

    for pos in 0..num_of_bits {
        let most_common_bit_value = find_most_common_bit_value_in_pos(&diagnostic_report, pos);
        gamma_rate += most_common_bit_value << pos;
        epsilon_rate_mask += 1 << pos;
    }

    (gamma_rate, gamma_rate ^ epsilon_rate_mask)
}

// Part 2
fn split_report(
    diagnostic_report: &Vec<usize>,
    bit: usize,
    pos: usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut nums_with_bit_at_pos = Vec::new();
    let mut the_rest = Vec::new();
    let mask = 1 << pos;
    let bit_at_pos = bit << pos;

    for d in diagnostic_report {
        if (*d) & mask == bit_at_pos {
            nums_with_bit_at_pos.push(*d);
        } else {
            the_rest.push(*d);
        }
    }

    (nums_with_bit_at_pos, the_rest)
}

fn find_o2_generator_rating(diagnostic_report: &Vec<usize>, pos: usize) -> usize {
    let most_common_bit = find_most_common_bit_value_in_pos(diagnostic_report, pos);
    let (o2_generator_split, _) = split_report(diagnostic_report, most_common_bit, pos);

    if o2_generator_split.len() == 1 {
        o2_generator_split[0]
    } else {
        find_o2_generator_rating(&o2_generator_split, pos - 1)
    }
}

fn find_co2_scrubber_rating(diagnostic_report: &Vec<usize>, pos: usize) -> usize {
    let least_common_bit = find_most_common_bit_value_in_pos(diagnostic_report, pos) ^ 0b1;
    let (co2_scrubber_split, _) = split_report(diagnostic_report, least_common_bit, pos);

    if co2_scrubber_split.len() == 1 {
        co2_scrubber_split[0]
    } else {
        find_co2_scrubber_rating(&co2_scrubber_split, pos - 1)
    }
}

fn find_o2_generator_and_co2_scrubber_ratings(
    diagnostic_report: Vec<usize>,
    num_of_bits: usize,
) -> (usize, usize) {
    (
        find_o2_generator_rating(&diagnostic_report, num_of_bits - 1),
        find_co2_scrubber_rating(&diagnostic_report, num_of_bits - 1),
    )
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_correct_bit() {
        let num = 0b11100;

        assert_eq!(0, get_bit_at_pos(num, 0));
        assert_eq!(0, get_bit_at_pos(num, 1));
        assert_eq!(1, get_bit_at_pos(num, 2));
        assert_eq!(1, get_bit_at_pos(num, 3));
        assert_eq!(1, get_bit_at_pos(num, 4));
    }

    #[test]
    fn finds_correct_most_common_bit() {
        let diagnostic_report = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let most_common_bit = find_most_common_bit_value_in_pos(&diagnostic_report, 4);

        assert_eq!(1, most_common_bit);
    }

    #[test]
    fn finds_correct_most_common_bit_when_tie() {
        let diagnostic_report = vec![0b10110, 0b10111];
        let most_common_bit = find_most_common_bit_value_in_pos(&diagnostic_report, 0);

        assert_eq!(1, most_common_bit);
    }

    #[test]
    fn part_1() {
        let diagnostic_report = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let (gamma_rate, epsilon_rate) = find_gamma_and_epsilon_rates(diagnostic_report, 5);

        assert_eq!(22, gamma_rate);
        assert_eq!(9, epsilon_rate);
    }

    #[test]
    fn splits_correctly() {
        let diagnostic_report = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let (split_with_ones, split_with_zeroes) = split_report(&diagnostic_report, 1, 4);

        assert_eq!(
            vec![0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001],
            split_with_ones
        );
        assert_eq!(
            vec![0b00100, 0b01111, 0b00111, 0b00010, 0b01010],
            split_with_zeroes
        );
    }

    #[test]
    fn part_2() {
        let diagnostic_report = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let (o2_generator_rating, co2_scrubber_rating) =
            find_o2_generator_and_co2_scrubber_ratings(diagnostic_report, 5);

        assert_eq!(23, o2_generator_rating);
        assert_eq!(10, co2_scrubber_rating);
    }
}
