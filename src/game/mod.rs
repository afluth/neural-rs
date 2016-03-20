pub use self::human::HumanPlayer;
pub use self::ai::AiPlayer;

mod ai;
mod human;
mod board;

use self::board::Mark;
use rand;

pub trait Player {
    fn set_mark(&mut self, board::Mark);
    fn play(&mut self, &mut board::View);
    fn game_result(&mut self, GameResult);
}

pub enum GameResult {
    Win,
    Loss,
    Tie,
}

pub fn play_game<P1: Player, P2: Player>(player1: &mut P1, player2: &mut P2) {
    let mut board = board::Board::new();

    player1.set_mark(Mark::X);
    player2.set_mark(Mark::O);

    let p2_rotation = rand::random();

    for i in 0..9 {
        let win;

        if (i % 2) == 0 {
            let mut view = board.get_view(board::Rotation::Bottom);
            win = take_turn(player1, player2, &mut view, i);
        } else {
            let mut view = board.get_view(p2_rotation);
            win = take_turn(player2, player1, &mut view, i);
        }

        if win {
            return;
        }

    }
    // Tie game
    player1.game_result(GameResult::Tie);
    player2.game_result(GameResult::Tie);
}


fn take_turn<P: Player, O: Player>(player: &mut P,
                                   opponent: &mut O,
                                   board: &mut board::View,
                                   turn: usize)
                                   -> bool {

    // Make move
    player.play(board);

    // Once possible, check for win
    if turn >= 5 && check_for_win(board) {
        player.game_result(GameResult::Win);
        opponent.game_result(GameResult::Loss);
        return true;
    }

    return false;
}

fn check_for_win(board: &board::View) -> bool {
    (board[0] != Mark::None && board[1] == board[0] && board[2] == board[0]) ||
    (board[3] != Mark::None && board[4] == board[3] && board[5] == board[3]) ||
    (board[6] != Mark::None && board[7] == board[6] && board[8] == board[6]) ||
    (board[0] != Mark::None && board[3] == board[0] && board[6] == board[0]) ||
    (board[1] != Mark::None && board[4] == board[1] && board[7] == board[1]) ||
    (board[2] != Mark::None && board[5] == board[2] && board[8] == board[2]) ||
    (board[0] != Mark::None && board[4] == board[0] && board[8] == board[0]) ||
    (board[2] != Mark::None && board[4] == board[2] && board[6] == board[2])
}
