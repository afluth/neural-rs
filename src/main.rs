extern crate rand;

mod neural;
mod game;

//use std::cmp::Ordering;
use game::*;
use rand::Rng;

const NUM_PLAYERS: usize = 200;
const NUM_SURVIVORS: usize = 50;
const GENERATIONS: usize = 100_000;

fn main() {
    let mut human = HumanPlayer::new();
    let mut players = Vec::with_capacity(NUM_PLAYERS);
    
    populate(&mut players);

    for generation in 0..GENERATIONS {
        repopulate(&mut players);
    
        find_fittest(&mut players);   

        {
            let best = &mut players[0];
            for w in best.neural_net.get_weights() {
                println!("{}", w);
            }
            println!("Generation: {}, Wins: {}, Ties: {}, Loses: {}, Mistakes: {}", 
                    generation, best.wins, best.ties, best.loses, best.mistakes);
            println!("-----------");

            if generation % 100 == 0 {
                play_game(best, &mut human);
            }
        }

        // Reset wins
        for player in players.iter_mut() {
            player.reset();
        }
    }
}

fn find_fittest(players: &mut Vec<AiPlayer>) {
    // Pit the players against each other
    let done: Vec<&mut AiPlayer> = Vec::with_capacity(NUM_PLAYERS);
    players.iter_mut()
        .fold(done, |mut done, player1| {
            for player2 in done.iter_mut() {
                // TODO Is player2 a &mut &mut AiPlayer here?
                play_game(player1, *player2);
                play_game(*player2, player1);
            }
            done.push(player1);
            return done;
        });
    
    // Sort by wins/loses/mistakes
    players.sort_by(|a, b| {
        b.get_rating().cmp(&a.get_rating())
        //let wins_ord = b.wins.cmp(&a.wins);
        //if wins_ord == Ordering::Equal {
        //    let mistakes_ord = a.mistakes.cmp(&b.mistakes);
        //    if mistakes_ord == Ordering::Equal {
        //        return a.loses.cmp(&b.loses);
        //    }
        //    return mistakes_ord;
        //}
        //return wins_ord;
    });

    // Keep only the best
    players.truncate(NUM_SURVIVORS);
}

fn populate(players: &mut Vec<AiPlayer>) {
    for _ in 0..NUM_PLAYERS {
        players.push(AiPlayer::new());
    }
}

fn repopulate(players: &mut Vec<AiPlayer>) {
    let mut rng = rand::thread_rng();

    // Repopulate any culled players
    for i in 0..(NUM_PLAYERS - players.len()) {
        
        let child = players[i % NUM_SURVIVORS]
                    .reproduce(&players[rng.gen_range(0, NUM_SURVIVORS)]);

        players.push(child);
    }
}
