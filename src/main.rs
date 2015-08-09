extern crate rand;

mod neural;
mod game;

use game::*;
use rand::Rng;

const NUM_PLAYERS: usize = 100;
const NUM_SURVIVORS: usize = 10;
const GENERATIONS: usize = 100;

fn main() {
    let mut players = Vec::with_capacity(NUM_PLAYERS);

    for _ in 0..GENERATIONS {
        repopulate(&mut players);
    
        find_fittest(&mut players);   
        
        println!("len: {}", players.len());
        for p in players.iter_mut() {
            println!("{}", p.wins);
            p.wins = 0;
        }
        println!("-----------");
    }


}

fn find_fittest(players: &mut Vec<Player>) {
    // Pit the players against each other
    let done: Vec<&mut Player> = Vec::with_capacity(NUM_PLAYERS);
    players.iter_mut()
        .fold(done, |mut done, player1| {
            for player2 in done.iter_mut() {
                play_game(player1, player2);
            }
            done.push(player1);
            return done;
        });
    
    // Sort by wins
    players.sort_by(|a, b| b.wins.cmp(&a.wins));

    // Keep only the best
    players.truncate(NUM_SURVIVORS);
}

fn repopulate(players: &mut Vec<Player>) {
    let mut rng = rand::thread_rng();

    // Repopulate any culled players
    for _ in 0..(NUM_PLAYERS - players.len()) {
        //players[index];
        
        println!("{}", rng.gen_range(0, NUM_SURVIVORS));
        //rng.choose(&players[0..NUM_SURVIVORS-1]);
        
    }
}
