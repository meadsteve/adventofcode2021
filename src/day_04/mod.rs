//use crate::helpers::DayData;
use crate::helpers::DayData;
use crate::AdventDay;

pub struct DayFour();

impl AdventDay for DayFour {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day04.txt");
        let mut game = parse_input(data.lines());
        let (last_call, winner) = game.play_and_get_winner();
        format!("Thingy: {}", winner.unmarked_sum() * last_call)
    }

    fn run_part_two(&self) -> String {
        todo!()
    }
}

fn parse_input<I: IntoIterator<Item = String>>(input: I) -> GameSet {
    let mut lines = input.into_iter();
    let numbers_to_call: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    lines.next();
    let mut cards = Vec::new();
    let mut card_numbers = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            cards.push(BingoCard::new(card_numbers));
            card_numbers = Vec::new();
        } else {
            card_numbers.push(
                line.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            )
        }
    }
    if !card_numbers.is_empty() {
        cards.push(BingoCard::new(card_numbers));
    }
    GameSet {
        numbers_to_call,
        cards,
    }
}

struct GameSet {
    numbers_to_call: Vec<usize>,
    cards: Vec<BingoCard>,
}

impl GameSet {
    fn play_and_get_winner(&mut self) -> (usize, BingoCard) {
        for number in &self.numbers_to_call {
            for card in &mut self.cards {
                card.mark(*number);
                if card.is_winner() {
                    return (*number, card.clone());
                }
            }
        }
        panic!("We assumed there'd always be a winner")
    }
}

#[derive(Clone)]
struct BingoCard {
    grid: Vec<Vec<BingoSquare>>,
    size: usize,
}

impl BingoCard {
    fn new(numbers: Vec<Vec<usize>>) -> Self {
        let mut grid = Vec::new();
        let size = numbers.len();
        for x in 0..size {
            let mut row = Vec::new();
            for y in 0..size {
                row.push(BingoSquare::new(numbers[x][y]));
            }
            grid.push(row);
        }
        Self { grid, size }
    }

    fn mark(&mut self, number: usize) {
        for x in 0..self.size {
            for y in 0..self.size {
                let square = &mut self.grid[x][y];
                if square.number == number {
                    square.mark()
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for x in 0..self.size {
            let mut row_win = true;
            let mut col_win = true;
            for y in 0..self.size {
                row_win = row_win && self.grid[x][y].marked;
                col_win = col_win && self.grid[y][x].marked;
            }
            if row_win || col_win {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|squares| {
                squares
                    .iter()
                    .filter(|square| !square.marked)
                    .map(|square| square.number)
            })
            .sum()
    }
}

#[derive(Clone)]
struct BingoSquare {
    pub number: usize,
    pub marked: bool,
}

impl BingoSquare {
    fn new(n: usize) -> Self {
        Self {
            number: n,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_blank_card_is_not_winning() {
        let card = BingoCard::new(vec![vec![1, 2], vec![3, 4]]);
        assert!(!card.is_winner());
    }

    #[test]
    fn test_a_row_can_win() {
        let mut card = BingoCard::new(vec![vec![1, 2], vec![3, 4]]);
        card.mark(1);
        card.mark(2);
        assert!(card.is_winner());
    }

    #[test]
    fn test_a_col_can_win() {
        let mut card = BingoCard::new(vec![vec![1, 2], vec![3, 4]]);
        card.mark(2);
        card.mark(4);
        assert!(card.is_winner());
    }

    #[test]
    fn test_parsing() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;
        let game = parse_input(input.lines().map(|x| x.to_string()));
        assert_eq!(game.numbers_to_call.len(), 27);
        assert_eq!(game.cards.len(), 3);
    }
}
