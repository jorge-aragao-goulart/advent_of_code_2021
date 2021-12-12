use std::error::Error;

mod days;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let day = days::get_day(config.day)?;
    println!("Solving Day {}", config.day);

    let p1_solution = day.part_1(&config.input_filename)?;
    println!("Part 1: {}", p1_solution);

    let p2_solution = day.part_2(&config.input_filename)?;
    println!("Part 2: {}", p2_solution);

    Ok(())
}

pub struct Config {
    day: i32,
    input_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 3 {
            return Err(Box::new(NotEnoughArguments));
        }

        let day: i32 = args[1].parse()?;
        let input_filename = args[2].clone();

        Ok(Config {
            day,
            input_filename,
        })
    }
}

// Implementation of NotEnoughArguments as an Error
use std::fmt;
#[derive(Debug)]
struct NotEnoughArguments;
impl fmt::Display for NotEnoughArguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not enough arguments")
    }
}
impl Error for NotEnoughArguments {}
