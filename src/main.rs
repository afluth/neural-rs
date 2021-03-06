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

	// Machine Learning
	if let Some(ref file_path) = args.examples {
		println!("Loading Examples...");

		let examples = neural::load_examples(file_path).unwrap();

		let mut net = neural::Network::with_dimensions(&[9, 9, 9]);
		println!("{:?}", net);
		
		let trainer = neural::Trainer::new(net, &examples);

		net = trainer.train();
		println!("{:?}", net);

		if args.human {
			let mut ai = AiPlayer::with_network(net);
			play_game(&mut ai, &mut human);
		}

	} else {
		// Evolution
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

    
}

struct Args {
    num_players: usize,
    human: bool,
    debug: bool,
	examples: Option<String>,
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
		.arg(clap::Arg::with_name("examples")
			.short("e")
			.long("examples")
			.help("File of training examples to use.")
			.takes_value(true)
			.value_name("FILE"))
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
		examples: matches.value_of("examples").map(|s| s.to_string()),
    }
}

