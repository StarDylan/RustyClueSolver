use std::{io::{self, Write, Read}, fs::File};

use crate::player_hand::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameState {
    pub player_hands: Vec<PlayerHand>
}

impl GameState {
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let serialized = serde_json::to_string(self)?;            
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn read_from_file(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut json_buf: String = String::new();
        file.read_to_string(&mut json_buf)?;
        let deserialized: GameState = serde_json::from_str(&json_buf)?;

        Ok(deserialized)
    }
}

/// Starts a new game
/// 
/// Requires your hand to be complete (no unknowns)
/// Other player names must be in order of play
/// 
/// Starting player dictates who is starting, 0 being you, 1 being the first listed
/// in the other_player_names, and so on.
fn new_game(self_hand: PlayerHand, other_player_hands: Vec<PlayerHand>, starting_player: usize) -> GameState {
    let mut player_hands: Vec<PlayerHand> = Vec::new();

    // Order the elements correctly
    for idx in 0..(other_player_hands.len() + 1) {
        let ordered_index = (starting_player + idx) % (other_player_hands.len() + 1);

        if ordered_index == 0 {
            player_hands.push(self_hand.clone());
        } else {
            player_hands.push(
                other_player_hands.get(ordered_index).unwrap().to_owned()
            );
        }
    }

    GameState { 
        player_hands
    }
}