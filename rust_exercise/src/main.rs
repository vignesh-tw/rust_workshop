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

    game.greeting();

    loop {
        game.board().draw();

        game.ask_user(&mut game.board().state(), player);

        if game.has_won(&game.board().state()) {
            game.board().draw();
            print!("Player '");
            player.print();
            println!("' won! \\(^.^)/");
            break;
        }

        if game.is_over(&game.board().state()) {
            game.board().draw();
            println!("All fields are used. No one won. (._.)");
            break;
        }

        player.switch();
    }
}
