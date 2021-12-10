use adventofcode2021::day_01::DayOne;
use adventofcode2021::day_02::DayTwo;
use adventofcode2021::day_03::DayThree;
use adventofcode2021::day_04::DayFour;
use adventofcode2021::day_05::DayFive;
use adventofcode2021::day_06::DaySix;
use adventofcode2021::day_07::DaySeven;
use adventofcode2021::day_10::DayTen;

use adventofcode2021::AdventDay;
use test_case::test_case;

#[test_case(Box::new(DayOne()), Some("depth increases: 1316"), Some("triplet increases: 1344"); "day one")]
#[test_case(Box::new(DayTwo()), Some("result: 2039912"), Some("result: 1942068080"); "day two")]
#[test_case(Box::new(DayThree()), Some("Weird thingy: 1082324"), Some("Weird thingy: 1353024"); "day three")]
#[test_case(Box::new(DayFour()), Some("Thingy: 23177"), Some("Thingy: 6804"); "day four")]
#[test_case(Box::new(DayFive()), Some("overlaps: 6007"), Some("overlaps: 19349"); "day five")]
#[test_case(Box::new(DaySix()), Some("Fish count: 387413"), None; "day six")]
#[test_case(Box::new(DaySeven()), Some("GOTO 337488"), Some("GOTO 89647695"); "day seven")]
#[test_case(Box::new(DayTen()), Some("Score: 366027"), None; "day ten")]
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
