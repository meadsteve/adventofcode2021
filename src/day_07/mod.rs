use crate::helpers::DayData;
use crate::AdventDay;
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct DaySeven();

impl AdventDay for DaySeven {
    fn run_part_one(&self) -> String {
        let crabs = DaySeven::craaaabs();
        format!(
            "GOTO {}",
            lowest_fuel_move(&crabs, basic_fuel_cost).fuel_cost
        )
    }

    fn run_part_two(&self) -> String {
        let crabs = DaySeven::craaaabs();
        format!(
            "GOTO {}",
            lowest_fuel_move(&crabs, triangular_fuel_cost).fuel_cost
        )
    }
}

impl DaySeven {
    fn craaaabs() -> Vec<Crab> {
        DayData::from_file_path("./data/day07.txt")
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|f| f.parse().unwrap())
            .collect()
    }
}

fn basic_fuel_cost(crabs: &[Crab], target: u64) -> u64 {
    crabs.iter().map(|c| c.fuel_cost(target)).sum()
}

fn triangular_fuel_cost(crabs: &[Crab], target: u64) -> u64 {
    crabs.iter().map(|c| c.triangular_fuel_cost(target)).sum()
}

fn lowest_fuel_move(crabs: &[Crab], fuel_func: fn(&[Crab], u64) -> u64) -> Move {
    let lowest = crabs.iter().min().unwrap().position();
    let highest = crabs.iter().max().unwrap().position();
    (lowest..=highest)
        .map(|target| Move {
            fuel_cost: fuel_func(crabs, target),
            target,
        })
        .min()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
struct Crab(u64);

#[derive(Debug, PartialEq, Eq)]
struct Move {
    target: u64,
    fuel_cost: u64,
}

impl PartialOrd<Self> for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fuel_cost.partial_cmp(&other.fuel_cost)
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fuel_cost.cmp(&other.fuel_cost)
    }
}

impl Crab {
    fn fuel_cost(&self, target: u64) -> u64 {
        (self.0 as i64 - target as i64).abs() as u64
    }

    fn triangular_fuel_cost(&self, target: u64) -> u64 {
        let steps = (self.0 as i64 - target as i64).abs() as u64;
        let mut fuel_cost = 0;
        let mut step_cost = 1;
        for _ in 0..steps {
            fuel_cost += step_cost;
            step_cost += 1;
        }
        fuel_cost
    }

    fn position(&self) -> u64 {
        self.0
    }
}

impl FromStr for Crab {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_work_out_the_fuel_cost_for_groups_of_crabs() {
        let crabs: Vec<Crab> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|i| i.parse::<Crab>().unwrap())
            .collect();
        assert_eq!(basic_fuel_cost(&crabs, 2), 37);
    }

    #[test]
    fn it_can_pick_the_lowest_fuel_position_for_the_crabs() {
        let crabs: Vec<Crab> = "16,1,2,0,4,2,7,1,2,14"
            .split(',')
            .map(|i| i.parse::<Crab>().unwrap())
            .collect();
        let best_move = lowest_fuel_move(&crabs, basic_fuel_cost);
        assert_eq!(best_move.fuel_cost, 37);
        assert_eq!(best_move.target, 2);
    }
}
