extern crate rand;
extern crate scoped_threadpool;
extern crate num_cpus;

mod neural;
mod game;
mod genetics;

use game::*;
use genetics::Evolution;

const NUM_PLAYERS: usize = 200;
//const GENERATIONS: usize = 1_000_000;

fn main() {
    let mut human = HumanPlayer::new();
	
	// New way
	let mut evolution = Evolution::<AiPlayer>::new(NUM_PLAYERS);
	
	evolution.evolve(100_000);
	{
		let best = &mut evolution.individuals[0];
		play_game(best, &mut human);
	}
	
}
