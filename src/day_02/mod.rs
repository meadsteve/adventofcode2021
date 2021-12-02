use std::error;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

use part_one::Position;

use crate::day_02::errors::ParseSubmarineCommandsError;
use crate::day_02::part_two::PositionAndAim;
use crate::helpers::DayData;
use crate::AdventDay;

mod errors;
mod part_one;
mod part_two;

pub struct DayTwo();

impl AdventDay for DayTwo {
    fn run_part_one(&self) -> String {
        let final_position = DayTwo::move_data()
            .iter()
            .fold(Position::new(), |position, sub_move| {
                position.apply_move(sub_move)
            });
        format!(
            "result: {}",
            final_position.horizontal * final_position.depth
        )
    }

    fn run_part_two(&self) -> String {
        let final_position = DayTwo::move_data()
            .iter()
            .fold(PositionAndAim::new(), |position, sub_move| {
                position.apply_move(sub_move)
            });
        format!(
            "result: {}",
            final_position.horizontal * final_position.depth
        )
    }
}

impl DayTwo {
    fn move_data() -> Vec<Move> {
        DayData::from_file_path("./data/day02.txt")
            .lines()
            .map(|line| line.parse::<Move>().unwrap())
            .collect()
    }
}

#[derive(PartialEq, Debug)]
pub enum Move {
    Forward(isize),
    Down(isize),
    Up(isize),
}

impl FromStr for Move {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("forward ") {
            Ok(Move::Forward(string_slice_as_int(s, 8)?))
        } else if s.starts_with("down ") {
            Ok(Move::Down(string_slice_as_int(s, 5)?))
        } else if s.starts_with("up ") {
            Ok(Move::Up(string_slice_as_int(s, 3)?))
        } else {
            Err(Box::new(ParseSubmarineCommandsError("WHAT?".to_string())))
        }
    }
}

fn string_slice_as_int(s: &str, start_pos: usize) -> Result<isize, ParseIntError> {
    let number_part = s.chars().skip(start_pos).collect::<String>();
    number_part.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_move_can_be_parsed() {
        assert_eq!("forward 5".parse::<Move>().unwrap(), Move::Forward(5));
    }

    #[test]
    fn down_move_can_be_parsed() {
        assert_eq!("down 7".parse::<Move>().unwrap(), Move::Down(7));
    }

    #[test]
    fn up_move_can_be_parsed() {
        assert_eq!("up 2".parse::<Move>().unwrap(), Move::Up(2));
    }
}
