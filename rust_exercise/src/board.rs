#[derive(Clone, Copy)]
pub struct Board {
    state: [char; 9],
}

impl Board {
    pub fn new() -> Self {
        Board {
            state: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }

    pub fn state(&self) -> [char;9] {
        self.state
    }

    pub fn update_state(&mut self, state: [char; 9]) {
        self.state = state;
    }
}