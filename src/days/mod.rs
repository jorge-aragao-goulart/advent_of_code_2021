use std::error::Error;

// Each day is divided into two parts.
// For each part, the solution must read and parse the input file, and return the answer formatted as a String
// (the user should be able to just copy&paste the returned String into the input field on Advent Of Code's website)
pub trait DaySolution {
    fn part_1(&self, _input_file: &String) -> Result<String, Box<dyn Error>> {
        unimplemented!();
    }

    fn part_2(&self, _input_file: &String) -> Result<String, Box<dyn Error>> {
        unimplemented!();
    }
}

// Each of these structures will implement their solution (DaySolution) in their own module
pub struct Day01;
mod day01;

pub struct Day02;
mod day02;

pub struct Day03;
mod day03;

pub struct Day04;
mod day04;

pub struct Day05;
mod day05;

pub struct Day06;
mod day06;

pub struct Day07;
mod day07;

pub struct Day08;
mod day08;

pub struct Day09;
mod day09;

pub struct Day10;
mod day10;

pub struct Day11;
mod day11;

pub struct Day12;
mod day12;

pub struct Day13;
mod day13;

pub struct Day14;
mod day14;

pub struct Day15;
mod day15;

pub struct Day16;
mod day16;

pub struct Day17;
mod day17;

pub struct Day18;
mod day18;

pub struct Day19;
mod day19;

pub struct Day20;
mod day20;

pub struct Day21;
mod day21;

pub struct Day22;
mod day22;

pub struct Day23;
mod day23;

pub struct Day24;
mod day24;

pub struct Day25;
mod day25;

// Find and return the correct solution for the given day
pub fn get_day(day: i32) -> Result<Box<dyn DaySolution>, InvalidDay> {
    match day {
        1 => Ok(Box::new(Day01)),
        2 => Ok(Box::new(Day02)),
        3 => Ok(Box::new(Day03)),
        4 => Ok(Box::new(Day04)),
        5 => Ok(Box::new(Day05)),
        6 => Ok(Box::new(Day06)),
        7 => Ok(Box::new(Day07)),
        8 => Ok(Box::new(Day08)),
        9 => Ok(Box::new(Day09)),
        10 => Ok(Box::new(Day10)),
        11 => Ok(Box::new(Day11)),
        12 => Ok(Box::new(Day12)),
        13 => Ok(Box::new(Day13)),
        14 => Ok(Box::new(Day14)),
        15 => Ok(Box::new(Day15)),
        16 => Ok(Box::new(Day16)),
        17 => Ok(Box::new(Day17)),
        18 => Ok(Box::new(Day18)),
        19 => Ok(Box::new(Day19)),
        20 => Ok(Box::new(Day20)),
        21 => Ok(Box::new(Day21)),
        22 => Ok(Box::new(Day22)),
        23 => Ok(Box::new(Day23)),
        24 => Ok(Box::new(Day24)),
        25 => Ok(Box::new(Day25)),
        _ => Err(InvalidDay(day)),
    }
}

// ... I am not happy with this implementation, but it does work for the small scope of Advent of Code
// I am open to hear what others way I could implement this, in order to further learn about Rust

// Implementation of InvalidDay as an Error
use std::fmt;
#[derive(Debug)]
pub struct InvalidDay(i32);
impl fmt::Display for InvalidDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "cannot request to solve day {}, must be 1 <= day <= 25",
            self.0
        )
    }
}
impl Error for InvalidDay {}
