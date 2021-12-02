use crate::day_02::Move;

pub struct Position {
    pub horizontal: isize,
    pub depth: isize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn apply_move(&self, sub_move: &Move) -> Position {
        match sub_move {
            Move::Forward(x) => Position {
                horizontal: (self.horizontal + x),
                ..*self
            },
            Move::Up(x) => Position {
                depth: self.depth - x,
                ..*self
            },
            Move::Down(x) => Position {
                depth: self.depth + x,
                ..*self
            },
        }
    }
}
