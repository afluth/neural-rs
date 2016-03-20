use super::{Player, GameResult};
use std::io::{BufRead, stdin};
use super::board;

pub struct HumanPlayer {
    mark: board::Mark,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer {
            mark: board::Mark::O,
        }
    }
}

impl Player for HumanPlayer {
    fn set_mark(&mut self, mark: board::Mark) {
        self.mark = mark;
    }   

    fn play(&mut self, board: &mut board::View) {
        println!("*****");
        println!("{}|{}|{}   0|1|2", board[0], board[1], board[2]);
        println!("-+-+-   -+-+-");
        println!("{}|{}|{}   3|4|5", board[3], board[4], board[5]);
        println!("-+-+-   -+-+-");
        println!("{}|{}|{}   6|7|8", board[6], board[7], board[8]);
        
        println!("Your move:");
        
        let stdin = stdin();
        
        for line in stdin.lock().lines() {
            let index: usize = line.unwrap().parse().unwrap();
       
            if board[index] == board::Mark::None {
                board[index] = self.mark;
                return;
            }
        }
    }

    fn game_result(&mut self, result: GameResult) {
        match result {
            GameResult::Win => println!("You Win!"),
            GameResult::Loss => println!("Oh noes! You lose!"),
            GameResult::Tie => println!("Cat game."),
        }
    }
}