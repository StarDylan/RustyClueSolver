use crate::player_hand::*;

struct GameState {
    player_hands: Vec<PlayerHand>
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