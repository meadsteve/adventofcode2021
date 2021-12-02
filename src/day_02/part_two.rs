use crate::day_02::Move;

pub struct PositionAndAim {
    pub horizontal: isize,
    pub depth: isize,
    pub aim: isize,
}

impl PositionAndAim {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn apply_move(&self, sub_move: &Move) -> Self {
        match sub_move {
            Move::Forward(x) => Self {
                horizontal: (self.horizontal + x),
                depth: (self.depth + (x * self.aim)),
                ..*self
            },
            Move::Up(x) => Self {
                aim: self.aim - x,
                ..*self
            },
            Move::Down(x) => Self {
                aim: self.aim + x,
                ..*self
            },
        }
    }
}
