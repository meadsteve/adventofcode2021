use adventofcode2021::day_01::DayOne;

use adventofcode2021::AdventDay;
use test_case::test_case;

#[test_case(Box::new(DayOne()), Some("depth increases: 1316"), Some("triplet increases: 1344"); "day one")]
fn test_the_days(
    solution: Box<dyn AdventDay>,
    day_one_sol: Option<&str>,
    day_two_sol: Option<&str>,
) {
    if let Some(expectation) = day_one_sol {
        assert_eq!(expectation, solution.run_part_one());
    }
    if let Some(expectation) = day_two_sol {
        assert_eq!(expectation, solution.run_part_two());
    }
}
