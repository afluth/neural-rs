use neural::*;

pub struct Player {
    neural_net: Network,
    mark: char,
    pub wins: u16,
}

impl Player {
    pub fn new() -> Player {
        Player {
            neural_net: Network::new(),
            mark: '*',
            wins: 0u16,
        }
    }

    fn play(&mut self, board: &[char; 9]) -> usize {
        let mut inputs = [0f32; 9];
        
        // Setup inputs based on the board state
        for (i, mark) in board.iter().enumerate() {
            
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

        return index;
    }

    fn reproduce(&self, partner: &Player) -> Player {
        return Player {
            neural_net: self.neural_net.reproduce(&partner.neural_net),
            mark: '*',
            wins: 0u16,
        }
    }
}

pub fn play_game(player1: &mut Player, player2: &mut Player) {
    let mut board = [' '; 9];

    // Assign marks
    player1.mark = 'X';
    player2.mark = 'O';
    
    for i in 0..9 {
        // Select the current player
        let player: &mut Player = if (i % 2) == 0 { 
            player1 
        } else { 
            player2 
        };

        // Make move        
        let player_move = player.play(&board);
        board[player_move] = player.mark;
        
        /*
        println!("*****");
        println!("{}|{}|{}", board[0], board[1], board[2]);
        println!("-+-+-");
        println!("{}|{}|{}", board[3], board[4], board[5]);
        println!("-+-+-");
        println!("{}|{}|{}", board[6], board[7], board[8]);
        */
        
        // Once possible, check for win
        if i >= 5 &&
           ((board[0] != ' ' && board[1] == board[0] && board[2] == board[0]) ||
           (board[3] != ' ' && board[4] == board[3] && board[5] == board[3]) ||
           (board[6] != ' ' && board[7] == board[6] && board[8] == board[6]) ||
           (board[0] != ' ' && board[3] == board[0] && board[6] == board[0]) ||
           (board[1] != ' ' && board[4] == board[1] && board[7] == board[1]) ||
           (board[2] != ' ' && board[5] == board[2] && board[8] == board[2]) ||
           (board[0] != ' ' && board[4] == board[0] && board[8] == board[0]) ||
           (board[2] != ' ' && board[4] == board[2] && board[6] == board[2]))
        {
            //println!("Winner!");
            player.wins += 1;
            break;
        }
    }
}
