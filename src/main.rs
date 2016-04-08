#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate rustc_serialize;
extern crate scoped_threadpool;

mod game;
mod genetics;
mod neural;
mod persistance;

use game::*;
use genetics::Evolution;

fn main() {
    let args = get_args();
    
    let mut human = HumanPlayer::new();

    // New way
    let mut evolution = Evolution::<AiPlayer>::new(args.num_players);

	if let Some(players) = persistance::load() {
	    println!("Existing state loaded!");
	    evolution.individuals = players
	}

    loop {
        evolution.evolve(100);
		
		persistance::save(&evolution.individuals).unwrap();
		
		if args.human {
        	let best = &mut evolution.individuals[0];
        
        	play_game(best, &mut human);
		}
    }
}

struct Args {
    num_players: usize,
    human: bool,
    debug: bool,
}

fn get_args() -> Args {
    let matches = clap::App::new("Neural")
        .version(crate_version!())
    	.about("Overly complicated Tic-Tac-Toe.")
    	.arg(clap::Arg::with_name("players")
    	    .short("p")
    		.long("players")
    		.help("Sets the number of players per generation.")
    		.takes_value(true)
    		.value_name("NUM")
    		.default_value("500"))
    	.arg(clap::Arg::with_name("human")
    		.short("H")
    		.long("human")
    		.help("Would the human like to play?"))
    	.arg(clap::Arg::with_name("debug")
    	    .long("debug")
    	    .help("Turn on debug output"))
    	.get_matches();
    	
    Args {
    	num_players: value_t_or_exit!(matches, "players", usize),
    	human: matches.is_present("human"),
    	debug: matches.is_present("debug"),
    }
}

