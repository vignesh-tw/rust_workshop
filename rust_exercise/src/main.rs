mod game;
mod board;
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
        game.draw(&game.board().state());

        // Ask for user input
        game.ask_user(&mut game.board().state(), player);

        // Check if a player won
        if game.has_won(&game.board().state()) {
            game.draw(&game.board().state());
            print!("Player '");
            game.print_player(&player.to_char());
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if game.is_over(&game.board().state()) {
            game.draw(&game.board().state());
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch player
        player = if player == Player::X { Player::O } else { Player::X }
    }
}
