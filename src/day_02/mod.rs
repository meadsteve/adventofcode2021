use std::error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use crate::day_02::errors::ParseSubmarineCommandsError;
use crate::helpers::DayData;
use crate::AdventDay;

mod errors;

pub struct DayTwo();

impl AdventDay for DayTwo {
    fn run_part_one(&self) -> String {
        let final_position = DayTwo::move_data().iter().fold(
            Position {
                horizontal: 0,
                depth: 0,
            },
            |position, sub_move| position.apply_move(sub_move),
        );
        format!(
            "result: {}",
            final_position.horizontal * final_position.depth
        )
    }

    fn run_part_two(&self) -> String {
        todo!()
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

struct Position {
    horizontal: isize,
    depth: isize,
}

impl Position {
    fn apply_move(&self, sub_move: &Move) -> Position {
        match sub_move {
            Move::Forward(x) => Position {
                horizontal: (self.horizontal + x),
                ..*self
            },
            Move::Up(x) => Position {
                depth: self.depth - x,
                ..*self
            },
            Move::Down(x) => Position {
                depth: self.depth + x,
                ..*self
            },
        }
    }
}

#[derive(PartialEq, Debug)]
enum Move {
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
