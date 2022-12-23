use crate::{game_state::GameState, player_hand::Card};

pub fn propagate_state(gs: &mut GameState){

    // Propogate Does not Haves, if someone could not respond
    for acc in gs.accusations.iter_mut() {
        let players_who_did_not_have_cards 
            = get_responding_players(
                acc.accuser_player_index, 
                acc.responding_player_index, 
                gs.player_hands.len()
            );

        for player_who_did_not_have_card_index in players_who_did_not_have_cards {
            
            let must_not_have_set = 
                &mut gs.player_hands
                    .get_mut(player_who_did_not_have_card_index)
                    .unwrap()
                    .must_not_have;


                must_not_have_set.insert(Card::RoomCard(acc.room.clone()));
                must_not_have_set.insert(Card::WeaponCard(acc.weapon.clone()));
                must_not_have_set.insert(Card::SuspectCard(acc.suspect.clone()));
        }

    }


    // TODO: If someone must have a card, everyone else must not have that card
}


/// Gets index of players who are between start and end, wrapping around if neccessary.
/// 
/// Does not include the endIdx or startIdx
/// 
/// If endIdx is None, then it gets all players except startIdx
pub fn get_responding_players(start_idx: usize,end_idx: Option<usize>, size: usize) -> Vec<usize> {
    
    let mut responding_players: Vec<usize> = Vec::new();
    
    for i in 0..(size-1) {
        let current_player_idx = (i + start_idx + 1) % size;

        // If at end, stop
        if end_idx.is_some() && current_player_idx == end_idx.unwrap() {
            return responding_players;
        }

        responding_players.push(current_player_idx);
    }

    return responding_players;
}