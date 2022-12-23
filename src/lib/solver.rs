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
/// Does not include the end_idx or start_idx
/// 
/// If end_idx is None, then it gets all players except start_idx
/// 
/// start_idx must be < size, same with end_idx.
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


#[cfg(test)]
mod tests {
    use std::{collections::HashSet, vec};

    use crate::{player_hand::{PlayerHand, Room, Suspect, Weapon}, accusation::Accusation};

    use super::*;

    #[test]
    fn test_propogate_state_not_responding_adds_does_not_haves() {

        let player_hands = vec![
            PlayerHand::new("p1".to_owned()),
            PlayerHand::new("p2".to_owned()),
            PlayerHand::new("p3".to_owned()),
            PlayerHand::new("p4".to_owned()),
        ];

        let accusations = vec![
            Accusation { 
                accuser_player_index: 0, 
                room: Room::Study, 
                suspect: Suspect::Peacock, 
                weapon: Weapon::Knife, 
                responding_player_index: Some(3), 
                card_shown: None
            }
        ];

        let mut gs = GameState {
            public_cards: HashSet::new(),
            player_hands: player_hands,
            self_index: 0,
            accusations: accusations,
        };

        propagate_state(&mut gs);

        for i in vec![0, 3] {
            assert_eq!(gs.player_hands.get(i).unwrap().must_not_have.len(), 0);
            assert_eq!(gs.player_hands.get(i).unwrap().must_have.len(), 0);
        }


        for i in 1..=2 {
            assert_eq!(gs.player_hands.get(i).unwrap().must_not_have.len(), 3);
            assert!(gs.player_hands.get(i).unwrap().must_not_have.contains(&Card::RoomCard(Room::Study)));
            assert!(gs.player_hands.get(i).unwrap().must_not_have.contains(&Card::SuspectCard(Suspect::Peacock)));
            assert!(gs.player_hands.get(i).unwrap().must_not_have.contains(&Card::WeaponCard(Weapon::Knife)));
        }
        
    }

    #[test]
    fn test_get_responding_players() {
        let players_between = get_responding_players(0, Some(4), 5);

        let expected_result = vec![1,2,3];

        assert_eq!(players_between.len(), expected_result.len());

        for it in players_between.iter().zip(expected_result.iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }
    }

    #[test]
    fn test_get_responding_players_wrap() {
        let players_between = get_responding_players(3, Some(1), 5);

        let expected_result = vec![4,0];

        assert_eq!(players_between.len(), expected_result.len());

        for it in players_between.iter().zip(expected_result.iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }
    }

    #[test]
    fn test_get_responding_players_none() {
        let players_between = get_responding_players(3,None, 5);

        let expected_result = vec![4,0,1,2];

        assert_eq!(players_between.len(), expected_result.len());

        for it in players_between.iter().zip(expected_result.iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }
    }
}