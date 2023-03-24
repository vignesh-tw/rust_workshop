use board::Board;
use game::Game;
use player::Player;

mod board;
mod game;
mod player;

fn main() {
    let board = Board::new();
    let mut player = Player::X;
    let mut game = Game::new(board, player);

    game.greeting();

    loop {
        game.board().draw();

        game.gets_input_from_current_player(&mut game.board().state());

        if game.is_won_by_any_player(&game.board().state()) {
            game.board().draw();
            print!("Player '");
            player.print();
            println!("' won! \\(^.^)/");
            break;
        }

        if game.is_over(&game.board().state()) {
            game.board().draw();
            println!("Match Draw! (._.)");
            break;
        }

        game.switch_player();
    }
}
