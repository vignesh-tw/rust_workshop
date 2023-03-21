mod game;

use game::Game;

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    let game = Game::new();

    // Welcome the player
    game.greeting();

    loop {
        // Draw the field
        game.draw(&state);

        // Ask for user input
        game.ask_user(&mut state, player);

        // Check if a player won
        if game.has_won(&state) {
            game.draw(&state);
            print!("Player '");
            game.print_player(&player);
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if game.is_over(&state) {
            game.draw(&state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
