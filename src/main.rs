extern crate rand;
extern crate scoped_threadpool;
extern crate num_cpus;

mod neural;
mod game;

use game::*;
use rand::Rng;
use scoped_threadpool::{Pool};

const NUM_PLAYERS: usize = 200;
const NUM_SURVIVORS: usize = 100;
const GENERATIONS: usize = 1_000_000;

fn main() {
    let mut human = HumanPlayer::new();
    let mut players = Vec::with_capacity(NUM_PLAYERS);
    
    populate(&mut players);

    for generation in 0..GENERATIONS {
        repopulate(&mut players);
        
        play_round_robin(&mut players);

        find_fittest(&mut players);

        {
            let best = &mut players[0];
            //for w in best.neural_net.weights {
            //    println!("{}", w);
            //}
            
            println!("Generation: {}, Wins: {}, Ties: {}, Loses: {}, Mistakes: {}", 
                    generation, best.wins, best.ties, best.loses, best.mistakes);
            println!("-----------");

            if generation > 0 && generation % 100_000 == 0 {
                play_game(best, &mut human);
            }
        }

        // Reset wins
        for player in players.iter_mut() {
            player.reset();
        }
    }
    //println!("Play time: {}", play_sw.elapsed());
    //println!("Repopulate time: {}", pop_sw.elapsed());
}

fn play_round_robin(players: &mut [AiPlayer]) {

    let mut pool = Pool::new(num_cpus::get() as u32);   
    bisect_players(&mut pool, players);   
}

fn bisect_players(pool: &mut Pool, players: &mut [AiPlayer]) {
    let length = players.len();
    let bisect = length - (length / 2);
    
    let (group1, group2) = players.split_at_mut(bisect);
    
    play_groups(pool, group1, group2);
    
    if bisect > 1 {
        bisect_players(pool, group1);
        bisect_players(pool, group2);
    }
}

fn play_groups(pool: &mut Pool, group1: &mut [AiPlayer], group2: &mut [AiPlayer]) {
    
    // This assumes group1 is larger if they aren't the same size
    assert!(group2.len() <= group1.len());
    for i in 0..group1.len() {
        let (front, back) = group1.split_at_mut(i);
        let group1_iter = back.iter_mut()
                              .chain(front.iter_mut());

        let pairs = group1_iter.zip(group2.iter_mut());

        pool.scoped(|scope| {
            for (player1, player2) in pairs {
                scope.execute(move || {
                    play_game(player1, player2);
                    play_game(player2, player1);
                });
            }
        });

    }    

}

/*
fn play_round_robin(players: &mut Vec<AiPlayer>) {
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
}
*/

fn find_fittest(players: &mut Vec<AiPlayer>) {
    // Sort by wins/loses/mistakes
    players.sort_by(|a, b| {
        b.get_rating().cmp(&a.get_rating())
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
