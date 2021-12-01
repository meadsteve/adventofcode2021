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
        todo!()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_something() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = number_of_increases(input);
        assert_eq!(result, 7);
    }
}
