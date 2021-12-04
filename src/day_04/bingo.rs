#[derive(Clone)]
pub struct BingoCard {
    grid: Vec<Vec<BingoSquare>>,
    size: usize,
}

impl BingoCard {
    pub fn new(numbers: Vec<Vec<usize>>) -> Self {
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

    pub fn mark(&mut self, number: usize) {
        for x in 0..self.size {
            for y in 0..self.size {
                let square = &mut self.grid[x][y];
                if square.number == number {
                    square.mark()
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
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

    pub fn unmarked_sum(&self) -> usize {
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
    use crate::day_04::*;

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
}
