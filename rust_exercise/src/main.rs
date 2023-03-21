mod game;
mod board;

use board::Board;
use game::Game;



fn main() {
    let mut board = Board::new();
    let mut player = 'X';

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
            game.print_player(&player);
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
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
