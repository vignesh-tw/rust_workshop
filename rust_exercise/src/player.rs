#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn to_char(&self) -> char {
        if *self == Self::X {
            'X'
        } else {
            'O'
        }
    }

    pub fn switch_player(&mut self) {
        *self = if *self == Player::X {
            Player::O
        } else {
            Player::X
        };
    }
}
