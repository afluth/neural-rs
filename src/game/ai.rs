use std::fmt;
use super::{board, Player, GameResult, play_game};
use genetics::Individual;
use neural::Network;

#[derive(RustcEncodable, RustcDecodable)]
pub struct AiPlayer {
    pub neural_net: Network,
    mark: board::Mark,
    pub wins: u16,
    pub loses: u16,
    pub ties: u16,
    pub mistakes: u16,
}

impl Player for AiPlayer {

    fn set_mark(&mut self, mark: board::Mark) {
        self.mark = mark;
    }   

    fn play(&mut self, board: &mut board::View) {
        let mut inputs = [0f32; 9];
        
        // Setup inputs based on the board state
        //for (i, mark) in board.iter().enumerate() {
        for i in 0..9 {
            let mark = board[i];
            
            if mark == self.mark {
                inputs[i] = -1f32;
            } else if mark != board::Mark::None {
                inputs[i] = 1f32;
            }
        }
        
        // Run it through the neural network
        let outputs = self.neural_net.calc(inputs);
        
        // Make a move
        let mut sorted_outputs = outputs.iter()
            .enumerate()
            .collect::<Vec<_>>();
        sorted_outputs.sort_by(|&(_, a), &(_, b)| {
            a.abs().partial_cmp(&b.abs()).unwrap()
        });
        
        for (i, _) in sorted_outputs {
            if board[i] == board::Mark::None {
                board[i] = self.mark;
                return;
            } else {
                self.mistakes += 1;
            }
        }
    }

    fn game_result(&mut self, result: GameResult) {
        match result {
            GameResult::Win => self.wins += 1,
            GameResult::Loss => self.loses += 1,
            GameResult::Tie => self.ties += 1,
        }
    }
}

impl fmt::Debug for AiPlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AiPlayer {{ wins: {}, loses: {}, ties: {}, mistakes: {} }}", 
            self.wins, self.loses, self.ties, self.mistakes)
    }
}

impl Individual for AiPlayer {

    fn new() -> AiPlayer {
        AiPlayer {
            neural_net: Network::new(),
            mark: board::Mark::None,
            wins: 0u16,
            loses: 0u16,
            ties: 0u16,
            mistakes: 0u16,
        }
    }
    
    fn reset(&mut self) {
        self.wins = 0;
        self.ties = 0;
        self.loses = 0;
        self.mistakes = 0;
    }

    fn get_rating(&self) -> i32 {
        (self.wins * 4 + self.ties * 4 - self.loses * 4 - self.mistakes) as i32
        //self.wins as i32
    }

    fn reproduce(&self, partner: &AiPlayer) -> AiPlayer {
        return AiPlayer {
            neural_net: self.neural_net.reproduce(&partner.neural_net),
            mark: board::Mark::None,
            wins: 0u16,
            loses: 0u16,
            ties: 0u16,
            mistakes: 0u16,
        }
    }
    
    fn compete(&mut self, other: &mut AiPlayer) {
        play_game(self, other);
        play_game(other, self);
    }
}