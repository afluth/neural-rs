use neural::*;
use std::io::{BufRead, stdin};

pub trait Player {
    fn set_mark(&mut self, char);
    fn play(&mut self, &mut [char; 9]);
    fn game_result(&mut self, GameResult);
}

enum GameResult {
    Win,
    Loss,
    Tie,
}

pub struct AiPlayer {
    pub neural_net: Network,
    mark: char,
    pub wins: u16,
    pub loses: u16,
    pub ties: u16,
    pub mistakes: u16,
}

impl AiPlayer {
    pub fn new() -> AiPlayer {
        AiPlayer {
            neural_net: Network::new(),
            mark: '*',
            wins: 0u16,
            loses: 0u16,
            ties: 0u16,
            mistakes: 0u16,
        }
    }

    pub fn reset(&mut self) {
        self.wins = 0;
        self.ties = 0;
        self.loses = 0;
        self.mistakes = 0;
    }

    pub fn get_rating(&self) -> i16 {
        (self.wins * 4 + self.ties * 6 - self.loses * 4 - self.mistakes) as i16
    }

    pub fn reproduce(&self, partner: &AiPlayer) -> AiPlayer {
        return AiPlayer {
            neural_net: self.neural_net.reproduce(&partner.neural_net),
            mark: '*',
            wins: 0u16,
            loses: 0u16,
            ties: 0u16,
            mistakes: 0u16,
        }
    }
}

pub struct HumanPlayer {
    mark: char,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        HumanPlayer {
            mark: '*',
        }
    }
}

impl Player for HumanPlayer {
    fn set_mark(&mut self, mark: char) {
        self.mark = mark;
    }   

    fn play(&mut self, board: &mut [char; 9]) {
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
       
            if board[index] == ' ' {
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

impl Player for AiPlayer {

    fn set_mark(&mut self, mark: char) {
        self.mark = mark;
    }   

    fn play(&mut self, board: &mut [char; 9]) {
        let mut inputs = [0f32; 9];
        
        // Setup inputs based on the board state
        for (i, mark) in board.iter().enumerate() {
            
            if mark == &self.mark {
                inputs[i] = -1f32;
            } else if mark != &' ' {
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
            if board[i] == ' ' {
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

pub fn play_game<P1: Player, P2: Player>
        (player1: &mut P1, player2: &mut P2) {
    let mut board = [' '; 9];
    player1.set_mark('X');
    player2.set_mark('O');

    for i in 0..9 {
        let mut win;

        if (i % 2) == 0 { 
            win = take_turn(player1, player2, &mut board, i);
        } else { 
            win = take_turn(player2, player1, &mut board, i); 
        }

        if win {
            return;
        }
    }
    // Tie game
    player1.game_result(GameResult::Tie);
    player2.game_result(GameResult::Tie);
}

fn take_turn<P: Player, O: Player>
        (player: &mut P, opponent: &mut O, 
         board: &mut [char; 9], turn: usize) -> bool {
    
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

fn check_for_win(board: &[char; 9]) -> bool {
    (board[0] != ' ' && board[1] == board[0] && board[2] == board[0]) ||
    (board[3] != ' ' && board[4] == board[3] && board[5] == board[3]) ||
    (board[6] != ' ' && board[7] == board[6] && board[8] == board[6]) ||
    (board[0] != ' ' && board[3] == board[0] && board[6] == board[0]) ||
    (board[1] != ' ' && board[4] == board[1] && board[7] == board[1]) ||
    (board[2] != ' ' && board[5] == board[2] && board[8] == board[2]) ||
    (board[0] != ' ' && board[4] == board[0] && board[8] == board[0]) ||
    (board[2] != ' ' && board[4] == board[2] && board[6] == board[2])
}
