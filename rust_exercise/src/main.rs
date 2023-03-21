mod game;

use game::Game;

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    let game = Game::new();

    // Welcome the player
    Game::greeting();

    loop {
        // Draw the field
        Game::draw(&state);

        // Ask for user input
        Game::ask_user(&mut state, player);

        // Check if a player won
        if Game::has_won(&state) {
            Game::draw(&state);
            print!("Player '");
            Game::print_player(&player);
            println!("' won! \\(^.^)/");
            break;
        }

        // Check if all fields are used
        if Game::is_over(&state) {
            Game::draw(&state);
            println!("All fields are used. No one won. (._.)");
            break;
        }

        // Switch player
        player = if player == 'X' { 'O' } else { 'X' }
    }
}
