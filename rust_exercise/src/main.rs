mod board;
mod game;
mod player;

use board::Board;
use game::Game;
use player::Player;

fn main() {
    let mut board = Board::new();
    let mut player = Player::X;

    let mut game = Game::new(board);

    // Welcome the player
    game.greeting();

    loop {
        // Draw the field
        game.board().draw();

        // Ask for user input
        game.ask_user(&mut game.board().state(), player);

        // Check if a player won
        if game.has_won(&game.board().state()) {
            game.board().draw();
            print!("Player '");
            player.print();
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if game.is_over(&game.board().state()) {
            game.board().draw();
            println!("All fields are used. No one won. (._.)");
            break;
        }

        player.switch();
    }
}
