extern crate rand;

mod neural;

use neural::*;

struct Player {
    neural_net: Network,
    mark: char,
}

impl Player {
    fn new(mark: char) -> Player {
        Player {
            neural_net: Network::new(),
            mark: mark,
        }
    }

    fn play(&mut self, mut board: [char; 9]) -> [char; 9] {
        let mut inputs = [0f32; 9];
        
        // Setup inputs based on the board state
        for m in (0..).zip(board.iter()) {
            let (i, mark) = m;
            
            if mark == &self.mark {
                inputs[i] = 1f32;
            } else if mark != &' ' {
                inputs[i] = -1f32;
            }
        }
        
        // Run it through the neural network
        let outputs = self.neural_net.calc(inputs);
        
        // Make a move
        let mut index = 0;
        let mut max_weight = 0f32;
        for i in 0..outputs.len() {
            if outputs[i] > max_weight && board[i] == ' ' {
                index = i;
                max_weight = outputs[i];
            }
        }
        board[index] = self.mark;

        return board;
    }
}

fn main() {
    let mut player1 = Player::new('X');
    let mut player2 = Player::new('O'); 
    play_game(&mut player1, &mut player2);
}

fn play_game(player1: &mut Player, player2: &mut Player) {
    let mut board = [' '; 9];
    
    for i in 0..9 {
        // Make move        
        if (i % 2) == 0 {
            board = player1.play(board);        
        } else {
            board = player2.play(board);
        }

        println!("*****");
        println!("{}|{}|{}", board[0], board[1], board[2]);
        println!("-+-+-");
        println!("{}|{}|{}", board[3], board[4], board[5]);
        println!("-+-+-");
        println!("{}|{}|{}", board[6], board[7], board[8]);
        
        // Check for win
        if (board[0] != ' ' && board[1] == board[0] && board[2] == board[0]) ||
           (board[3] != ' ' && board[4] == board[3] && board[5] == board[3]) ||
           (board[6] != ' ' && board[7] == board[6] && board[8] == board[6]) ||
           (board[0] != ' ' && board[3] == board[0] && board[6] == board[0]) ||
           (board[1] != ' ' && board[4] == board[1] && board[7] == board[1]) ||
           (board[2] != ' ' && board[5] == board[2] && board[8] == board[2]) ||
           (board[0] != ' ' && board[4] == board[0] && board[8] == board[0]) ||
           (board[2] != ' ' && board[4] == board[2] && board[6] == board[2])
        {
            println!("Winner!");
            break;
        }
    }
}
