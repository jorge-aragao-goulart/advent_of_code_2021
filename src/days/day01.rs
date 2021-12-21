use super::{Day01, DaySolution};
use std::error::Error;

impl DaySolution for Day01 {
    fn part_1(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let measurements = read_input(input_file);
        Ok(count_increases(measurements).to_string())
    }

    fn part_2(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let measurements = read_input(input_file);
        Ok(count_sliding_window_increases(measurements, 3).to_string())
    }
}

// Shared
use std::fs;
fn read_input(input_file: &String) -> Vec<i32> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|i_str| i_str.parse::<i32>().unwrap())
        .collect()
}

// Part 1
fn count_increases(measurements: Vec<i32>) -> i32 {
    let mut n_increases = 0;
    let mut prev_measurement = measurements[0];

    for curr_measurement in measurements {
        if curr_measurement > prev_measurement {
            n_increases += 1;
        }
        prev_measurement = curr_measurement;
    }

    n_increases
}

// Part 2
fn count_sliding_window_increases(measurements: Vec<i32>, sliding_window_size: usize) -> i32 {
    let mut sum_of_measurements: Vec<i32> = Vec::new();

    for i in 0..=measurements.len() - sliding_window_size {
        let mut sum = 0;
        for j in i..i + sliding_window_size {
            sum += measurements[j]
        }
        sum_of_measurements.push(sum);
    }

    count_increases(sum_of_measurements)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_increases(measurements));
    }

    #[test]
    fn part_2() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_sliding_window_increases(measurements, 3));
    }
}
