use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayOne();

impl AdventDay for DayOne {
    fn run_part_one(&self) -> String {
        let depths = DayOne::depth_data();
        let answer = number_of_increases(depths);
        format!("depth increases: {}", answer)
    }

    fn run_part_two(&self) -> String {
        let depths = DayOne::depth_data();
        let triplets = to_triplet_sums(depths);
        let answer = number_of_increases(triplets);
        format!("triplet increases: {}", answer)
    }
}

impl DayOne {
    fn depth_data() -> Vec<isize> {
        DayData::from_file_path("./data/day01.txt")
            .lines()
            .map(|line| {
                line.parse::<isize>()
                    .expect("All of the depths should have been numbers")
            })
            .collect()
    }
}

fn number_of_increases(depths: Vec<isize>) -> usize {
    let firsts = depths.iter();
    let seconds = depths.iter().skip(1);
    seconds
        .zip(firsts)
        .filter(|(second, first)| second > first)
        .count()
}

fn to_triplet_sums(depths: Vec<isize>) -> Vec<isize> {
    let firsts = depths.iter();
    let seconds = depths.iter().skip(1);
    let thirds = depths.iter().skip(2);
    thirds
        .zip(seconds)
        .zip(firsts)
        .map(|((x, y), z)| *x + *y + *z)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_count_the_number_of_increases() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = number_of_increases(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_can_convert_to_triplet_sums() {
        let input = vec![199, 200, 208, 210];
        let result = to_triplet_sums(input);
        assert_eq!(result, vec![607, 618]);
    }
}
