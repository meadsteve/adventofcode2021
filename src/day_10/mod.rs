use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayTen();

impl AdventDay for DayTen {
    fn run_part_one(&self) -> String {
        let score: usize = DayData::from_file_path("./data/day10.txt")
            .lines()
            .map(|s| valid_line(&s))
            .map(|r| r.to_points())
            .sum();
        format!("Score: {}", score)
    }

    fn run_part_two(&self) -> String {
        todo!()
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

struct BracketStack(Vec<Bracket>);

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
    Ok,
    InvalidLine(BracketMistake),
}

impl LineCheckResult {
    fn to_points(&self) -> usize {
        match &self {
            Self::Ok => 0,
            Self::InvalidLine(mistake) => mistake.to_points(),
        }
    }
}

fn valid_line(line: &str) -> LineCheckResult {
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
    LineCheckResult::Ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_for_balanced_lines() {
        assert_eq!(valid_line("()"), LineCheckResult::Ok);
        assert_eq!(valid_line("[<>({}){}[([])<>]]"), LineCheckResult::Ok);
        assert_eq!(valid_line("{()()()}"), LineCheckResult::Ok);
    }

    #[test]
    fn it_returns_the_mistakes_for_invalid_lines() {
        let expected = LineCheckResult::InvalidLine(BracketMistake {
            expected: Bracket::Square,
            got: Bracket::Squirrely,
        });
        assert_eq!(valid_line("{([(<{}[<>[]}>{[]{[(<()>"), expected);
    }
}
