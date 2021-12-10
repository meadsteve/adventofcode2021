use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayTen();

impl AdventDay for DayTen {
    fn run_part_one(&self) -> String {
        let score: usize = DayData::from_file_path("./data/day10.txt")
            .lines()
            .map(|s| validate_line(&s))
            .map(|r| r.to_mistake_points())
            .sum();
        format!("Score: {}", score)
    }

    fn run_part_two(&self) -> String {
        let mut all_scores: Vec<usize> = DayData::from_file_path("./data/day10.txt")
            .lines()
            .map(|s| validate_line(&s))
            .filter(|r| !matches!(r, LineCheckResult::InvalidLine(_)))
            .map(|r| r.to_autocomplete_points())
            .collect();
        all_scores.sort_unstable();
        println!(
            "total: {}, midpoint: {}",
            all_scores.len(),
            all_scores.len() / 2
        );
        let score = all_scores.get(all_scores.len() / 2).unwrap();
        format!("Score: {}", score)
    }
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Squirrely,
    Round,
    Square,
    Triangular,
}

#[derive(Debug, PartialEq)]
struct BracketMistake {
    expected: Bracket,
    got: Bracket,
}

impl BracketMistake {
    fn to_points(&self) -> usize {
        match self.got {
            Bracket::Round => 3,
            Bracket::Square => 57,
            Bracket::Squirrely => 1197,
            Bracket::Triangular => 25137,
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpenCloseResult {
    Good,
    Bad(BracketMistake),
}

#[derive(Debug, PartialEq)]
struct BracketStack(Vec<Bracket>);

impl BracketStack {
    fn to_points(&self) -> usize {
        let mut score = 0;
        for i in self.0.iter().rev() {
            score *= 5;
            score += match i {
                Bracket::Round => 1,
                Bracket::Square => 2,
                Bracket::Squirrely => 3,
                Bracket::Triangular => 4,
            }
        }
        score
    }
}

impl BracketStack {
    fn new() -> BracketStack {
        BracketStack(Vec::new())
    }

    fn open(&mut self, opening: Bracket) -> OpenCloseResult {
        self.0.push(opening);
        OpenCloseResult::Good
    }

    fn close(&mut self, closing: Bracket) -> OpenCloseResult {
        let expected = self.0.pop().expect("You closed too much.");
        if closing == expected {
            return OpenCloseResult::Good;
        }
        OpenCloseResult::Bad(BracketMistake {
            expected,
            got: closing,
        })
    }
}

#[derive(Debug, PartialEq)]
enum LineCheckResult {
    Ok { remaining: BracketStack },
    InvalidLine(BracketMistake),
}

impl LineCheckResult {
    fn to_mistake_points(&self) -> usize {
        match &self {
            Self::Ok { remaining: _ } => 0,
            Self::InvalidLine(mistake) => mistake.to_points(),
        }
    }

    fn to_autocomplete_points(&self) -> usize {
        match &self {
            Self::Ok { remaining: stack } => stack.to_points(),
            Self::InvalidLine(_) => 0,
        }
    }
}

fn validate_line(line: &str) -> LineCheckResult {
    let mut stack = BracketStack::new();
    for c in line.chars() {
        let result = match c {
            '(' => stack.open(Bracket::Round),
            '[' => stack.open(Bracket::Square),
            '{' => stack.open(Bracket::Squirrely),
            '<' => stack.open(Bracket::Triangular),
            ')' => stack.close(Bracket::Round),
            ']' => stack.close(Bracket::Square),
            '}' => stack.close(Bracket::Squirrely),
            '>' => stack.close(Bracket::Triangular),
            _ => panic!("ONLY BRACKETS"),
        };
        if let OpenCloseResult::Bad(mistake) = result {
            return LineCheckResult::InvalidLine(mistake);
        }
    }
    LineCheckResult::Ok { remaining: stack }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_for_balanced_lines() {
        let success = LineCheckResult::Ok {
            remaining: BracketStack::new(),
        };
        assert_eq!(validate_line("()"), success);
        assert_eq!(validate_line("[<>({}){}[([])<>]]"), success);
        assert_eq!(validate_line("{()()()}"), success);
    }

    #[test]
    fn it_returns_the_mistakes_for_invalid_lines() {
        let expected = LineCheckResult::InvalidLine(BracketMistake {
            expected: Bracket::Square,
            got: Bracket::Squirrely,
        });
        assert_eq!(validate_line("{([(<{}[<>[]}>{[]{[(<()>"), expected);
    }

    #[test]
    fn it_scores_incomplete_lines() {
        assert_eq!(validate_line("()").to_autocomplete_points(), 0);
        assert_eq!(
            validate_line("{([(<{}[<>[]}>{[]{[(<()>").to_autocomplete_points(),
            0
        );
        assert_eq!(
            validate_line("[({(<(())[]>[[{[]{<()<>>").to_autocomplete_points(),
            288957
        );
        assert_eq!(
            validate_line("<{([{{}}[<[[[<>{}]]]>[]]").to_autocomplete_points(),
            294
        );

        assert_eq!(
            validate_line("{<[[]]>}<{[{[{[]{()[[[]").to_autocomplete_points(),
            995444
        );
        assert_eq!(
            validate_line("[(()[<>])]({[<{<<[]>>(").to_autocomplete_points(),
            5566
        );
        assert_eq!(
            validate_line("(((({<>}<{<{<>}{[]{[]{}").to_autocomplete_points(),
            1480781
        );
    }
}
