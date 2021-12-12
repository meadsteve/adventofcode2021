use crate::helpers::DayData;
use crate::AdventDay;
use std::collections::HashMap;

pub struct DayEleven();

impl AdventDay for DayEleven {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day11.txt");
        let lines = data.lines();
        let mut ocotopopulation = DayEleven::cave_from_lines(lines);
        for _day in 0..100 {
            ocotopopulation.run_day();
        }
        format!("flashes: {}", ocotopopulation.flashes)
    }

    fn run_part_two(&self) -> String {
        let data = DayData::from_file_path("./data/day11.txt");
        let lines = data.lines();
        let mut ocotopopulation = DayEleven::cave_from_lines(lines);
        let mut day = 1;
        loop {
            match ocotopopulation.run_day() {
                100 => break,
                _ => day += 1,
            };
        }
        format!("Days until megaflash: {}", day)
    }
}

impl DayEleven {
    fn cave_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Cave {
        let mut ocotopopulation = Cave::new();
        for (y, line) in lines.into_iter().enumerate() {
            for (x, energy) in line.chars().enumerate() {
                let energy = energy.to_digit(10).unwrap();
                ocotopopulation.add_octopus(Position::new(x, y), Octopus::new(energy as usize));
            }
        }
        ocotopopulation
    }
}

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

#[derive(Debug, PartialEq, Eq)]
struct Octopus {
    energy: usize,
    has_pulsed: bool,
}

enum ChargeResult {
    Energizing,
    Flashed,
}

impl Octopus {
    fn new(energy: usize) -> Self {
        Self {
            energy: energy,
            has_pulsed: false,
        }
    }

    fn add_to_energy(&mut self) -> ChargeResult {
        self.energy += 1;
        match (self.energy, self.has_pulsed) {
            (energy_level, false) if energy_level > 9 => {
                self.has_pulsed = true;
                ChargeResult::Flashed
            }
            (_, _) => ChargeResult::Energizing,
        }
    }

    fn end_of_day(&self) -> Self {
        match self.has_pulsed {
            false => Self {
                energy: self.energy,
                has_pulsed: false,
            },
            true => Self {
                energy: 0,
                has_pulsed: false,
            },
        }
    }
}

struct Cave {
    octopii: HashMap<Position, Octopus>,
    flashes: usize,
}

impl Cave {
    pub fn new() -> Cave {
        Cave {
            octopii: HashMap::new(),
            flashes: 0,
        }
    }

    pub fn add_octopus(&mut self, pos: Position, oct: Octopus) {
        self.octopii.insert(pos, oct);
    }

    pub fn run_day(&mut self) -> usize {
        self.energize_day();
        let daily_flash_count = self.octopii.iter().filter(|(_, o)| o.has_pulsed).count();
        self.octopii = self
            .octopii
            .iter()
            .map(|(pos, oct)| (*pos, oct.end_of_day()))
            .collect();
        daily_flash_count
    }

    fn energize_day(&mut self) {
        let mut energy_increases: Vec<Position> = self.octopii.keys().cloned().collect();
        loop {
            let flashes = self.increase_energy(&energy_increases);
            self.flashes += flashes.len();
            energy_increases = flashes.iter().flat_map(Position::neighbours).collect();
            if energy_increases.is_empty() {
                break;
            }
        }
    }

    fn increase_energy(&mut self, energy_increases: &[Position]) -> Vec<Position> {
        let mut flashes: Vec<Position> = vec![];
        for position in energy_increases {
            if let Some(octopus) = self.octopii.get_mut(&position) {
                match octopus.add_to_energy() {
                    ChargeResult::Energizing => (),
                    ChargeResult::Flashed => flashes.push(*position),
                }
            }
        }
        flashes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let start = "11111
19991
19191
19991
11111";
        let after_step_one = "34543
40004
50005
40004
34543";
        let after_step_two = "45654
51115
61116
51115
45654";
        let mut cave = DayEleven::cave_from_lines(start.lines().map(|s| s.to_string()));
        let expected_efter_step_one =
            DayEleven::cave_from_lines(after_step_one.lines().map(|s| s.to_string()));
        let expected_efter_step_two =
            DayEleven::cave_from_lines(after_step_two.lines().map(|s| s.to_string()));

        cave.run_day();
        assert_eq!(cave.octopii, expected_efter_step_one.octopii);

        cave.run_day();
        assert_eq!(cave.octopii, expected_efter_step_two.octopii);
    }
}
