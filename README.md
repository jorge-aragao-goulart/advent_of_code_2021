# Advent Of Code 2021 (in Rust!)

My implementations for the [Advent of Code 2021](https://adventofcode.com/2021/) challenges.

This year I decided to use Rust to tackle these challenges, as a way of learning the language.  
Not going for any special milestones, just trying to get as many stars as I can.

# Usage

To run (with Cargo), just use  
```
$ cargo run <DAY> <FILE>
```  
where DAY is the day you wish to run (a value between 1 and 25) and FILE is the path to a file containing the input for that day.

# Project Structure

For each day X, there is an implementation of `DaySolution` for the related struct `DayX`.  
Each day's implementation is located in a different module, each with their own file at `src/days/day__.rs`, so e.g.: if you want to check my implementation for day 16, you'll find it at `src/days/day16.rs`.
