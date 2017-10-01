use game::AiPlayer;
use rustc_serialize::json;
use std::error;
use std::fs;
use std::io::prelude::*;

const STATE_FILE: &'static str = "state.json";

pub fn load() -> Option<Vec<AiPlayer>> {
    
    if let Ok(mut file) = fs::File::open("state.json") {
        
        let mut encoded = String::new();
        
        if let Err(_) = file.read_to_string(&mut encoded) {
            return None;
        }
        
        if let Ok(players) = json::decode(&encoded) {
            Some(players)
        } else {
            None
        }
    } else {
        None
    }
    
}

pub fn save(players: &Vec<AiPlayer>) ->  Result<(), Box<error::Error>> {
    
    let encoded = json::encode(players)?.into_bytes();
    
    let mut file = fs::OpenOptions::new()
    	.write(true)
    	.create(true)
    	.truncate(true)
    	.open(STATE_FILE)?
        .write_all(&encoded)?;
    
    Ok(())
}