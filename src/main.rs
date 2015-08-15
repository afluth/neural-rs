extern crate rand;

mod neural;
mod game;

use game::*;
use rand::Rng;

const NUM_PLAYERS: usize = 1000;
const NUM_SURVIVORS: usize = 100;
const GENERATIONS: usize = 100;

fn main() {
    let mut players = Vec::with_capacity(NUM_PLAYERS);

    populate(&mut players);

    for _ in 0..GENERATIONS {
        repopulate(&mut players);
    
        find_fittest(&mut players);   
        /*
        println!("len: {}", players.len());
        for p in players.iter_mut().take(10) {
            println!("{}", p.wins);
            p.wins = 0;
        }
        */
        {
            let best = &players[0];
            println!("Wins: {}", best.wins);
            //for w in best.neural_net.get_weights() {
            //    println!("{}", w);
            //}
            println!("-----------");
        }

        // Reset wins
        for player in players.iter_mut() {
            player.wins = 0;
        }
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

fn populate(players: &mut Vec<Player>) {
    for _ in 0..NUM_PLAYERS {
        players.push(Player::new());
    }
}

fn repopulate(players: &mut Vec<Player>) {
    let mut rng = rand::thread_rng();

    // Repopulate any culled players
    for i in 0..(NUM_PLAYERS - players.len()) {
        //rng.choose(&players[0..NUM_SURVIVORS-1]);
        
        let child = players[i % NUM_SURVIVORS]
                    .reproduce(&players[rng.gen_range(0, NUM_SURVIVORS)]);

        players.push(child);
    }
}
