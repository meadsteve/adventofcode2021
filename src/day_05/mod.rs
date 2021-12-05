use crate::helpers::DayData;
use crate::AdventDay;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct DayFive();

impl AdventDay for DayFive {
    fn run_part_one(&self) -> String {
        let overlaps = solve_part_one(DayData::from_file_path("./data/day05.txt").lines());
        format!("overlaps: {}", overlaps)
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

fn solve_part_one<I: IntoIterator<Item = String>>(lines: I) -> usize {
    lines_into_a_crossover_map(lines)
        .iter()
        .filter(|(_, count)| **count > 1)
        .count()
}

fn lines_into_a_crossover_map<I: IntoIterator<Item = String>>(lines: I) -> HashMap<Point, usize> {
    lines
        .into_iter()
        .map(|l| l.parse::<Line>().unwrap())
        .filter(|l| !l.is_diag())
        .flat_map(|l| l.points())
        .fold(HashMap::new(), |mut counts, point| {
            let total = counts.entry(point).or_insert(0);
            *total += 1;
            counts
        })
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(',').collect::<Vec<&str>>()[..] {
            [x, y] => Ok(Point {
                x: x.parse::<usize>()?,
                y: y.parse::<usize>()?,
            }),
            _ => panic!("ahhhhh"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diag(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.start.x == self.end.x {
            for y in min(self.start.y, self.end.y)..=max(self.start.y, self.end.y) {
                points.push(Point { x: self.start.x, y })
            }
        } else if self.start.y == self.end.y {
            for x in min(self.start.x, self.end.x)..=max(self.start.x, self.end.x) {
                points.push(Point { y: self.start.y, x })
            }
        } else {
            panic!("Diaganol")
        }
        points
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(" -> ").collect::<Vec<&str>>()[..] {
            [start, end] => Ok(Line {
                start: start.parse()?,
                end: end.parse()?,
            }),
            _ => panic!("aaaah"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_can_be_made_from_the_strings() {
        assert_eq!(
            "0,9 -> 5,9".parse::<Line>().unwrap(),
            Line {
                start: Point { x: 0, y: 9 },
                end: Point { x: 5, y: 9 }
            }
        );
    }

    #[test]
    fn a_line_can_be_turned_into_its_points() {
        let line = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 2, y: 9 },
        };
        assert_eq!(
            line.points(),
            vec![
                Point { x: 0, y: 9 },
                Point { x: 1, y: 9 },
                Point { x: 2, y: 9 }
            ]
        );
    }

    #[test]
    fn another_line_can_be_turned_into_its_points() {
        let line = Line {
            start: Point { x: 3, y: 9 },
            end: Point { x: 3, y: 7 },
        };
        assert_eq!(
            line.points(),
            vec![
                Point { x: 3, y: 7 },
                Point { x: 3, y: 8 },
                Point { x: 3, y: 9 }
            ]
        );
    }
    #[test]
    fn test_part_one() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = solve_part_one(input.lines().map(|x| x.to_string()));
        assert_eq!(result, 5);
    }
}
