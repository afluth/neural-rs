extern crate num_cpus;
extern crate rand;
extern crate rustc_serialize;
extern crate scoped_threadpool;

mod neural;
mod game;
mod genetics;

use game::*;
use genetics::Evolution;
use rustc_serialize::json;
use std::io::prelude::*;
use std::fs;

const NUM_PLAYERS: usize = 200;

fn main() {
    let mut human = HumanPlayer::new();

    // New way
    let mut evolution = Evolution::<AiPlayer>::new(NUM_PLAYERS);

	if let Some(players) = load() {
	    println!("Existing state loaded!");
	    evolution.individuals = players
	}

    loop {
        evolution.evolve(100);
		
		save(&evolution.individuals).unwrap();
		
        let best = &mut evolution.individuals[0];
        
        play_game(best, &mut human);

    }

}


fn load() -> Option<Vec<AiPlayer>> {
    
    if let Ok(mut file) = fs::File::open("state.json") {
        
        let mut encoded = String::new();
        file.read_to_string(&mut encoded).unwrap();
        
        let players = json::decode(&encoded).unwrap();
        
        Some(players)
    } else {
        None
    }
    
}

fn save(players: &Vec<AiPlayer>) ->  Result<(), Box<std::error::Error>> {
    
    let mut file = try!(fs::OpenOptions::new()
    	.write(true)
    	.create(true)
    	.truncate(true)
    	.open("state.json"));
    
    let encoded = try!(json::encode(players));
    
    try!(file.write_all(&encoded.into_bytes()));
    
    Ok(())
}
