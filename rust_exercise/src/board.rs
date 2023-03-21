use crate::player::Player;
extern crate termcolor;
use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

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

    pub fn state(&self) -> [char; 9] {
        self.state
    }

    pub fn update_state(&mut self, state: [char; 9]) {
        self.state = state;
    }

    pub fn draw(&self) {
        println!("\n");

        for i in (0..3).rev() {
            let offset = i * 3;

            print!("-------------\n| ");
            self.print(&self.state[offset]);
            print!(" | ");
            self.print(&self.state[offset + 1]);
            print!(" | ");
            self.print(&self.state[offset + 2]);
            println!(" |");
        }

        println!("-------------");
    }

    pub fn print(&self, state: &char) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if state == &'X' {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                .unwrap();
        } else if state == &'O' {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
        }

        write!(&mut stdout, "{}", state).unwrap();
        stdout.reset().unwrap();
    }
}
