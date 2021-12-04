use multibit_counter::MultiBitCounter;

use crate::helpers::DayData;
use crate::AdventDay;

mod bit_counter;
mod multibit_counter;
mod ratings;

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
        let data = DayData::from_file_path("./data/day03.txt");
        let oxygen_rating = ratings::find_oxygen_rating(data.lines().collect());
        let scrubber = ratings::find_scrubber_rating(data.lines().collect());
        let answer = usize::from_str_radix(&oxygen_rating, 2).unwrap()
            * usize::from_str_radix(&scrubber, 2).unwrap();
        format!("Weird thingy: {}", answer)
    }
}

trait PartOneSolver {
    fn gamma_rate(&self) -> String;
    fn epsilon_rate(&self) -> String;
}
