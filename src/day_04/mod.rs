use bingo::BingoCard;

use crate::helpers::DayData;
use crate::AdventDay;

mod bingo;

pub struct DayFour();

impl AdventDay for DayFour {
    fn run_part_one(&self) -> String {
        let data = DayData::from_file_path("./data/day04.txt");
        let mut game = parse_input(data.lines());
        let (last_call, winner) = game.play_and_get_winner();
        format!("Thingy: {}", winner.unmarked_sum() * last_call)
    }

    fn run_part_two(&self) -> String {
        let data = DayData::from_file_path("./data/day04.txt");
        let mut game = parse_input(data.lines());
        let (last_call, last_winner) = game.find_last_winner();
        format!("Thingy: {}", last_winner.unmarked_sum() * last_call)
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

    fn find_last_winner(&mut self) -> (usize, BingoCard) {
        let mut cards: Vec<BingoCard> = self.cards.to_vec();
        for number in &self.numbers_to_call {
            for card in &mut cards {
                card.mark(*number);
            }
            if cards.len() == 1 {
                if cards[0].is_winner() {
                    return (*number, cards.pop().unwrap());
                }
            } else {
                cards = cards.iter().filter(|c| !c.is_winner()).cloned().collect();
            }
        }
        panic!("We assumed there'd always be a winner")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
