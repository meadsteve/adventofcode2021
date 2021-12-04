use multibit_counter::MultiBitCounter;

use crate::helpers::DayData;
use crate::AdventDay;

mod bit_counter;
mod multibit_counter;

pub struct DayThree();

impl AdventDay for DayThree {
    fn run_part_one(&self) -> String {
        let mut counter = MultiBitCounter::<12>::new();
        for line in DayData::from_file_path("./data/day03.txt").lines() {
            counter.count(&line);
        }
        let answer = usize::from_str_radix(&counter.gamma_rate(), 2).unwrap()
            * usize::from_str_radix(&counter.epsilon_rate(), 2).unwrap();
        format!("Weird thingy: {}", answer)
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

trait PartOneSolver {
    fn gamma_rate(&self) -> String;
    fn epsilon_rate(&self) -> String;
}
