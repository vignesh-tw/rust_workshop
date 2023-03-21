mod game;

use game::Game;

struct Board {
    state: [char; 9],
}

impl Board {
    fn new() -> Self {
        Board {
            state: ['1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut player = 'X';

    let game = Game::new();

    // Welcome the player
    game.greeting();

    loop {
        // Draw the field
        game.draw(&board.state);

        // Ask for user input
        game.ask_user(&mut board.state, player);

        // Check if a player won
        if game.has_won(&board.state) {
            game.draw(&board.state);
            print!("Player '");
            game.print_player(&player);
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if game.is_over(&board.state) {
            game.draw(&board.state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
