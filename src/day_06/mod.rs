use crate::helpers::DayData;
use crate::AdventDay;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct DaySix();

impl AdventDay for DaySix {
    fn run_part_one(&self) -> String {
        let input = DayData::from_file_path("./data/day06.txt")
            .lines()
            .next()
            .unwrap();
        let mut fishes: Vec<LanternFish> = input.split(',').map(|f| f.parse().unwrap()).collect();
        for _day in 0..80 {
            fishes = a_day_passes_for_the_school(fishes);
        }
        format!("Fish count: {}", fishes.len())
    }

    fn run_part_two(&self) -> String {
        // TODO: build numbers based on population age and just use that rather than
        // simulating each fish
        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum FishDayResult {
    Baby,
    NoBaby,
}

#[derive(Debug, PartialEq)]
struct LanternFish(usize);

impl FromStr for LanternFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LanternFish(s.parse()?))
    }
}

impl LanternFish {
    fn new_baby() -> LanternFish {
        LanternFish(8)
    }

    fn next_day(&mut self) -> FishDayResult {
        match self.0 {
            0 => {
                self.0 = 6;
                FishDayResult::Baby
            }
            _ => {
                self.0 -= 1;
                FishDayResult::NoBaby
            }
        }
    }
}

fn a_day_passes_for_the_school(mut fishes: Vec<LanternFish>) -> Vec<LanternFish> {
    for fishdex in 0..fishes.len() {
        match fishes[fishdex].next_day() {
            FishDayResult::Baby => fishes.push(LanternFish::new_baby()),
            FishDayResult::NoBaby => (),
        };
    }
    fishes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lantern_fish_can_be_made_from_a_string_number() {
        assert_eq!("7".parse::<LanternFish>().unwrap(), LanternFish(7));
    }

    #[test]
    fn a_lantern_fish_counts_down_to_the_day_it_gives_birth() {
        let mut fish = LanternFish(5);
        let result = fish.next_day();
        assert_eq!(result, FishDayResult::NoBaby);
        assert_eq!(fish, LanternFish(4));
    }

    #[test]
    fn a_lantern_fish_gives_birth_on_day_zero() {
        let mut fish = LanternFish(0);
        let result = fish.next_day();
        assert_eq!(result, FishDayResult::Baby);
        assert_eq!(fish, LanternFish(6));
    }

    fn school_as_a_string(school: Vec<LanternFish>) -> String {
        school
            .iter()
            .map(|f| f.0.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn the_next_day_happens_for_a_school_of_fish() {
        let fishes: Vec<LanternFish> = "3,4,3,1,2".split(',').map(|f| f.parse().unwrap()).collect();
        let next_fishes = a_day_passes_for_the_school(fishes);
        assert_eq!(school_as_a_string(next_fishes), "2,3,2,0,1")
    }

    #[test]
    fn a_new_generation_can_be_made() {
        let fishes: Vec<LanternFish> = "3,4,3,1,2".split(',').map(|f| f.parse().unwrap()).collect();
        let next_fishes = a_day_passes_for_the_school(fishes);
        let next_next_fishes = a_day_passes_for_the_school(next_fishes);
        assert_eq!(school_as_a_string(next_next_fishes), "1,2,1,6,0,8")
    }
}
