pub mod day_01;
pub mod day_02;
mod helpers;

use std::error::Error;
use std::fmt::{Display, Formatter};

pub use crate::day_01::DayOne;
use crate::day_02::DayTwo;
use std::str::FromStr;
use structopt::StructOpt;

pub trait AdventDay {
    fn run_part_one(&self) -> String;
    fn run_part_two(&self) -> String;
}

enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = &'static str;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        let day: u8 = day.parse().map_err(|_| "Must be a number")?;
        match day {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => Err("Only 1 or 2 is valid"),
        }
    }
}

#[derive(StructOpt)]
pub struct CliArgs {
    #[structopt(short, long)]
    day: i32,
    #[structopt(short, long)]
    part: Part,
}

#[derive(Debug)]
pub struct UnimplementedSolutionError(String);

impl Display for UnimplementedSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for UnimplementedSolutionError {}

pub fn run(args: &CliArgs, mut writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "Hello, advent of code")?;

    let solution: &(dyn AdventDay) = match args.day {
        1 => &DayOne(),
        2 => &DayTwo(),
        _ => {
            return Err(Box::new(UnimplementedSolutionError(
                "That day has not been done".to_string(),
            )))
        }
    };
    let result = run_day(solution, &args.part);
    writeln!(writer, "{}", result)?;
    Ok(())
}

fn run_day(solution: &dyn AdventDay, part: &Part) -> String {
    match part {
        Part::One => solution.run_part_one(),
        Part::Two => solution.run_part_two(),
    }
}
