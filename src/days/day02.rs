use super::{Day02, DaySolution};
use std::error::Error;

impl DaySolution for Day02 {
    fn part_1(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let mut submarine = SimpleSubmarine::new();
        let commands = read_input(input_file);
        submarine.process_all(commands);
        Ok((submarine.horizontal_pos * submarine.depth).to_string())
    }

    fn part_2(&self, input_file: &String) -> Result<String, Box<dyn Error>> {
        let mut submarine = ComplicatedSubmarine::new();
        let commands = read_input(input_file);
        submarine.process_all(commands);
        Ok((submarine.horizontal_pos * submarine.depth).to_string())
    }
}

// Shared
#[derive(Debug, PartialEq)]
enum SubmarineCommand {
    Forward(i32),
    Down(i32),
    Up(i32),
}
use std::str::FromStr;
impl FromStr for SubmarineCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(' ').collect();

        let dist_fromstr = coords[1].parse::<i32>().expect("Bad int value for command");
        match coords[0] {
            "forward" => Ok(SubmarineCommand::Forward(dist_fromstr)),
            "down" => Ok(SubmarineCommand::Down(dist_fromstr)),
            "up" => Ok(SubmarineCommand::Up(dist_fromstr)),
            _ => Err(String::from("Unknown command")),
        }
    }
}

use std::fs;
fn read_input(input_file: &String) -> Vec<SubmarineCommand> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|cmd_str| SubmarineCommand::from_str(cmd_str).expect("File contains invalid command"))
        .collect()
}

trait Submarine {
    fn process(&mut self, command: SubmarineCommand);
    fn process_all(&mut self, commands: Vec<SubmarineCommand>) {
        for cmd in commands {
            self.process(cmd);
        }
    }
}

// Part 1
struct SimpleSubmarine {
    horizontal_pos: i32,
    depth: i32,
}
impl SimpleSubmarine {
    fn new() -> SimpleSubmarine {
        SimpleSubmarine {
            horizontal_pos: 0,
            depth: 0,
        }
    }

    fn forward(&mut self, dist: i32) {
        self.horizontal_pos += dist;
    }

    fn dive(&mut self, dist: i32) {
        self.depth += dist;
    }
}
impl Submarine for SimpleSubmarine {
    fn process(&mut self, command: SubmarineCommand) {
        match command {
            SubmarineCommand::Forward(dist) => {
                self.forward(dist);
            }
            SubmarineCommand::Down(dist) => {
                self.dive(dist);
            }
            SubmarineCommand::Up(dist) => {
                self.dive(-dist);
            }
        }
    }
}

// Part 2
struct ComplicatedSubmarine {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}
impl ComplicatedSubmarine {
    fn new() -> ComplicatedSubmarine {
        ComplicatedSubmarine {
            horizontal_pos: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn change_aim(&mut self, delta: i32) {
        self.aim += delta;
    }

    fn forward(&mut self, dist: i32) {
        self.horizontal_pos += dist;
        self.depth += self.aim * dist;
    }
}
impl Submarine for ComplicatedSubmarine {
    fn process(&mut self, command: SubmarineCommand) {
        match command {
            SubmarineCommand::Down(delta) => {
                self.change_aim(delta);
            }
            SubmarineCommand::Up(delta) => {
                self.change_aim(-delta);
            }
            SubmarineCommand::Forward(dist) => {
                self.forward(dist);
            }
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_commands() {
        let forward_5 = SubmarineCommand::from_str("forward 5").unwrap();
        let down_8 = SubmarineCommand::from_str("down 8").unwrap();
        let up_3 = SubmarineCommand::from_str("up 3").unwrap();

        assert_eq!(SubmarineCommand::Forward(5), forward_5);
        assert_eq!(SubmarineCommand::Down(8), down_8);
        assert_eq!(SubmarineCommand::Up(3), up_3);
    }

    #[test]
    fn part_1() {
        let mut sub = SimpleSubmarine::new();
        let cmds = vec![
            SubmarineCommand::Forward(5),
            SubmarineCommand::Down(5),
            SubmarineCommand::Forward(8),
            SubmarineCommand::Up(3),
            SubmarineCommand::Down(8),
            SubmarineCommand::Forward(2),
        ];

        sub.process_all(cmds);
        assert_eq!(150, sub.horizontal_pos * sub.depth);
    }

    #[test]
    fn part_2() {
        let mut sub = ComplicatedSubmarine::new();
        let cmds = vec![
            SubmarineCommand::Forward(5),
            SubmarineCommand::Down(5),
            SubmarineCommand::Forward(8),
            SubmarineCommand::Up(3),
            SubmarineCommand::Down(8),
            SubmarineCommand::Forward(2),
        ];

        sub.process_all(cmds);
        assert_eq!(900, sub.horizontal_pos * sub.depth);
    }
}
