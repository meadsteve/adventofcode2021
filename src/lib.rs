pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_09;
pub mod day_10;
pub mod day_11;
mod helpers;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use structopt::StructOpt;

use crate::day_01::DayOne;
use crate::day_02::DayTwo;
use crate::day_03::DayThree;
use crate::day_04::DayFour;
use crate::day_05::DayFive;
use crate::day_06::DaySix;
use crate::day_07::DaySeven;
use crate::day_09::DayNine;
use crate::day_10::DayTen;
use crate::day_11::DayEleven;

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
        3 => &DayThree(),
        4 => &DayFour(),
        5 => &DayFive(),
        6 => &DaySix(),
        7 => &DaySeven(),
        // TODO: day 8
        9 => &DayNine(),
        10 => &DayTen(),
        11 => &DayEleven(),
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
