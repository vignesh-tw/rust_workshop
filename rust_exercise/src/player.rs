extern crate termcolor;

use std::io::Write;

use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn switch(&mut self) {
        *self = if *self == Player::X {
            Player::O
        } else {
            Player::X
        };
    }

    pub fn print(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if *self == Player::X {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                .unwrap();
        } else if *self == Player::O {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
        }

        write!(&mut stdout, "{}", self.to_char()).unwrap();
        stdout.reset().unwrap();
    }
}

#[cfg(test)]
mod player_tests {

    use super::Player;

    #[test]
    fn should_switch_player() {
        let mut player = Player::X;

        player.switch();

        assert_eq!(player, Player::O);
    }
}