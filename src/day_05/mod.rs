use crate::helpers::DayData;
use crate::AdventDay;
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
        let overlaps = solve_part_two(DayData::from_file_path("./data/day05.txt").lines());
        format!("overlaps: {}", overlaps)
    }
}

fn solve_part_one<I: IntoIterator<Item = String>>(lines: I) -> usize {
    lines_into_a_crossover_map(
        lines
            .into_iter()
            .map(|l| l.parse::<Line>().unwrap())
            .filter(|l| !l.is_diag()),
    )
    .iter()
    .filter(|(_, count)| **count > 1)
    .count()
}

fn solve_part_two<I: IntoIterator<Item = String>>(lines: I) -> usize {
    lines_into_a_crossover_map(lines.into_iter().map(|l| l.parse::<Line>().unwrap()))
        .iter()
        .filter(|(_, count)| **count > 1)
        .count()
}

fn lines_into_a_crossover_map<I: IntoIterator<Item = Line>>(lines: I) -> HashMap<Point, usize> {
    lines
        .into_iter()
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

#[derive(Debug, Copy, Clone)]
struct Vector {
    x: isize,
    y: isize,
}

impl Point {
    fn add_vector(self, vector: Vector) -> Point {
        // Lets pretend this can never go below zero
        Point {
            x: (self.x as isize + vector.x) as usize,
            y: (self.y as isize + vector.y) as usize,
        }
    }
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
        let mut point = self.start;
        let vector = Vector {
            x: direction(self.start.x, self.end.x),
            y: direction(self.start.y, self.end.y),
        };
        while point != self.end {
            points.push(point);
            point = point.add_vector(vector);
        }
        points.push(self.end);
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

fn direction(start: usize, end: usize) -> isize {
    if start == end {
        0
    } else {
        let diff = end as isize - start as isize;
        diff / diff.abs()
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
                Point { x: 3, y: 9 },
                Point { x: 3, y: 8 },
                Point { x: 3, y: 7 }
            ]
        );
    }

    #[test]
    fn a_diagonal_line_can_make_points_too() {
        let line = Line {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };
        assert_eq!(
            line.points(),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 }
            ]
        );
    }

    #[test]
    fn another_diagonal_line_can_make_points_too() {
        let line = Line {
            start: Point { x: 9, y: 7 },
            end: Point { x: 7, y: 9 },
        };
        assert_eq!(
            line.points(),
            vec![
                Point { x: 9, y: 7 },
                Point { x: 8, y: 8 },
                Point { x: 7, y: 9 }
            ]
        );
    }

    fn input_for_tests() -> String {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .to_string()
    }

    #[test]
    fn test_part_one() {
        let result = solve_part_one(input_for_tests().lines().map(|x| x.to_string()));
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(input_for_tests().lines().map(|x| x.to_string()));
        assert_eq!(result, 12);
    }
}
