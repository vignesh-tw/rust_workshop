extern crate termcolor;

use crate::{board::Board, player::Player};

use self::termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Game { board }
    }

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn greeting(&self) {
        println!(
            "\nRust TicTacToe\n\
         --------------\n\
         A simple game written in the rust programming language.\n\
         Code is available at: https://github.com/flofriday/tictactoe"
        )
    }

    pub fn print_player(&self, player: &char) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if player == &'X' {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
                .unwrap();
        } else if player == &'O' {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
                .unwrap();
        }

        write!(&mut stdout, "{}", player).unwrap();
        stdout.reset().unwrap();
    }

    pub fn draw(&self, state: &[char]) {
        println!("\n");

        for i in (0..3).rev() {
            let offset = i * 3;

            print!("-------------\n| ");
            self.print_player(&state[offset]);
            print!(" | ");
            self.print_player(&state[offset + 1]);
            print!(" | ");
            self.print_player(&state[offset + 2]);
            println!(" |");
        }

        println!("-------------");
    }

    pub fn ask_user(&mut self, state: &mut [char; 9], player: Player) {
        loop {
            print!("Player '");
            self.print_player(&player.to_char());
            println!("', enter a number: ");

            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                println!("Couldn't read line! Try again.");
                continue;
            }

            if let Ok(number) = input.trim().parse::<usize>() {
                if number < 1 || number > 9 {
                    println!("The field number must be between 1 and 9.");
                    continue;
                }

                let number = number - 1;

                if state[number] == 'X' || state[number] == 'O' {
                    print!("This field is already taken by '");
                    self.print_player(&state[number]);
                    println!("'.");
                    continue;
                }

                state[number] = player.to_char();

                self.board.update_state(*state);
                break;
            } else {
                println!("Only numbers are allowed.");
                continue;
            }
        }
    }

    pub fn has_won(&self, state: &[char]) -> bool {
        for tmp in 0..3 {
            if state[tmp] == state[tmp + 3] && state[tmp] == state[tmp + 6] {
                return true;
            }

            let tmp = tmp * 3;

            if state[tmp] == state[tmp + 1] && state[tmp] == state[tmp + 2] {
                return true;
            }
        }

        if (state[0] == state[4] && state[0] == state[8])
            || (state[2] == state[4] && state[2] == state[6])
        {
            return true;
        }

        false
    }

    #[inline(always)]
    pub fn is_over(&self, state: &[char]) -> bool {
        state.iter().all(|&v| v == 'X' || v == 'O')
    }
}
