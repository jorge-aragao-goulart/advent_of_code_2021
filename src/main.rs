use advent_of_code_2021::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_err| {
        println!("Usage: advent_of_code_2021 <DAY> <FILE>");
        println!("Run the solution for Advent of Code 2021 DAY, using FILE contents as input.");
        println!("Example: advent_of_code_2021 1 ./input/day01.txt");
        println!("");
        println!("Check https://adventofcode.com/2021/ to learn more about Advent of Code 2021!");

        process::exit(1);
    });

    if let Err(e) = advent_of_code_2021::run(config) {
        println!("Error while running solution: {}", e);

        process::exit(1);
    }
}
