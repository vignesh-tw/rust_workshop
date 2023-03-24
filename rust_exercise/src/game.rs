use board::BoardPosition;
use board::BoardPosition::{NotOccupied, Occupied};

use crate::{board::Board, player::Player};

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


    pub fn ask_user(&mut self, state: &mut [BoardPosition; 9], player: Player) {
        loop {
            print!("Player '");
            player.print();
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
                match state[number] {
                    NotOccupied(_) => {}
                    Occupied(player) =>
                        if player == Player::X || player == Player::O {
                            print!("This field is already taken by '");
                            player.print();
                            println!("'.");
                            continue;
                        }
                }

                state[number] = Occupied(player);

                self.board.update_state(*state);
                break;
            } else {
                println!("Only numbers are allowed.");
                continue;
            }
        }
    }

    pub fn has_won(&self, state: &[BoardPosition]) -> bool {
        for board_position in 0..3 {
            if state[board_position] == state[board_position + 3] && state[board_position] == state[board_position + 6] {
                return true;
            }

            let board_position = board_position * 3;

            if state[board_position] == state[board_position + 1] && state[board_position] == state[board_position + 2] {
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
    pub fn is_over(&self, state: &[BoardPosition]) -> bool {
        state.iter().all(|&v| v == Occupied(Player::X) || v == Occupied(Player::O))
    }
}

#[cfg(test)]
mod game_tests {
    use std::io::BufRead;
    use std::io::Write;

    use board::BoardPosition::{NotOccupied, Occupied};
    use Player::{O, X};

    use crate::board::Board;

    use super::Game;

    #[test]
    fn should_return_true_if_game_has_been_won_by_either_of_the_player() {
        let state = [
            Occupied(X),
            Occupied(X),
            Occupied(X),
            NotOccupied(4),
            NotOccupied(5),
            NotOccupied(6),
            NotOccupied(7),
            NotOccupied(8),
            NotOccupied(9),
        ];
        let mut board = Board::new();
        board.update_state(state);
        let game = Game::new(board);

        let game_won = game.has_won(&state);

        assert!(game_won);
    }

    #[test]
    fn should_return_false_if_game_has_not_been_won_by_either_of_the_player() {
        let state = [
            Occupied(X),
            Occupied(O),
            Occupied(X),
            NotOccupied(4),
            NotOccupied(5),
            NotOccupied(6),
            NotOccupied(7),
            NotOccupied(8),
            NotOccupied(9),
        ];
        let mut board = Board::new();
        board.update_state(state);
        let game = Game::new(board);

        let game_won = game.has_won(&state);

        assert!(!game_won);
    }

    #[test]
    fn should_return_true_if_game_is_over() {
        let state = [
            Occupied(X),
            Occupied(O),
            Occupied(X),
            Occupied(X),
            Occupied(O),
            Occupied(X),
            Occupied(O),
            Occupied(X),
            Occupied(O),
        ];
        let mut board = Board::new();
        board.update_state(state);
        let game = Game::new(board);

        let game_over = game.is_over(&state);

        assert!(game_over);
    }

    #[test]
    fn should_return_false_if_game_is_not_over() {
        let state = [
            Occupied(X),
            Occupied(O),
            Occupied(X),
            Occupied(X),
            Occupied(O),
            Occupied(X),
            Occupied(O),
            Occupied(X),
            NotOccupied(9)
        ];
        let mut board = Board::new();
        board.update_state(state);
        let game = Game::new(board);

        let game_over = game.is_over(&state);

        assert!(!game_over);
    }
}
