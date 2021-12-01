use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayOne();

impl AdventDay for DayOne {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day01.txt");
        let depths: Vec<isize> = data.lines().map(|x| x.parse::<isize>().unwrap()).collect();
        let answer = number_of_increases(depths);
        format!("depth increases: {}", answer)
    }

    fn run_part_two(&self) -> String {
        let data = DayData::from_file_path("./data/day01.txt");
        let depths: Vec<isize> = data.lines().map(|x| x.parse::<isize>().unwrap()).collect();
        let triplets = to_triplet_sums(depths);
        let answer = number_of_increases(triplets);
        format!("triplet increases: {}", answer)
    }
}

fn number_of_increases(depths: Vec<isize>) -> usize {
    let mut result = 0;
    let mut iter = depths.iter().peekable();
    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            if next > &current {
                result += 1;
            }
        }
    }
    result
}

fn to_triplet_sums(depths: Vec<isize>) -> Vec<isize> {
    let mut results = vec![];
    let mut firsts = depths.iter();
    let mut seconds = depths.iter().skip(1);
    let thirds = depths.iter().skip(2);
    for third in thirds {
        if let Some(second) = seconds.next() {
            if let Some(first) = firsts.next() {
                results.push(*first + *second + *third);
            }
        }
    }
    results
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
