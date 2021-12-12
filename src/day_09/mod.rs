use crate::helpers::DayData;
use crate::AdventDay;
use std::collections::HashMap;

pub struct DayNine();

impl AdventDay for DayNine {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day09.txt");
        let map = DayNine::heightmap_from_lines(data.lines());
        format!("lowpoint score: {}", DayNine::score_lowpoints(&map))
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

impl DayNine {
    fn heightmap_from_lines<I: IntoIterator<Item = String>>(lines: I) -> HeightMap {
        let mut map = HeightMap::new();
        for (y, line) in lines.into_iter().enumerate() {
            for (x, energy) in line.chars().enumerate() {
                let height = Height(energy.to_digit(10).unwrap() as usize);
                map.map_height(Position::new(x, y), height);
            }
        }
        map
    }

    fn score_lowpoints(map: &HeightMap) -> usize {
        map.lowpoints().iter().map(|d| d.height.0 + 1).sum()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Height(usize);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn neighbours(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if x != 0 || y != 0 {
                    positions.push(Position {
                        x: self.x + x,
                        y: self.y + y,
                    })
                }
            }
        }
        positions
    }
}

struct HeightMap(HashMap<Position, Height>);

struct DataPoint {
    #[allow(dead_code)]
    pos: Position,
    height: Height,
}

impl HeightMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn map_height(&mut self, pos: Position, height: Height) {
        self.0.insert(pos, height);
    }

    fn lowpoints(&self) -> Vec<DataPoint> {
        self.0
            .keys()
            .filter(|&p| self.is_lowpoint(p))
            .cloned()
            .map(|p| DataPoint {
                pos: p,
                height: self.0.get(&p).unwrap().clone(),
            })
            .collect()
    }

    fn is_lowpoint(&self, pos: &Position) -> bool {
        let point_height = self.0.get(pos).expect("that's off the map");
        let neighbours = pos.neighbours();
        neighbours.iter().all(|n| {
            if let Some(neighbour_height) = self.0.get(n) {
                neighbour_height >= point_height
            } else {
                true
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = DayNine::heightmap_from_lines(map.lines().map(|s| s.to_string()));
        assert_eq!(DayNine::score_lowpoints(&map), 15);
    }
}
