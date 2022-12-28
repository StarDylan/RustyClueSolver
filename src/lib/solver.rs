use std::collections::HashSet;

use crate::game_state::GameState;
use crate::cards::*;
use crate::errors::*;


/// Applies logicial consequences that must be true.
/// 
/// GameState must be valid in order to run propagate_state
pub fn propagate_state(gs: &mut GameState) -> Result<()>{

    // -> Must have
    // If a player shows a card, they must have that card
    for acc in gs.accusations.iter() {
        if acc.card_shown.is_none() {
            continue;
        }

        gs.player_hands
            .get_mut(acc.responding_player_index.unwrap())
            .unwrap()
            .must_have
            .insert(acc.card_shown.clone().unwrap());
    }

    // -> Does not haves
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


    // Since the next two blocks might impact each other,
    // we keep running them until no more changes occur.
    let mut changed = true;
    while changed {
        changed = false;

        // Must Haves -> Does not haves
        // If someone must have a card, everyone else must not have that card
        let mut all_must_haves: HashSet<Card> = HashSet::new();

        for player_hand in gs.player_hands.iter() {
            all_must_haves.extend(player_hand.must_have.clone().into_iter());
        }

        for player_hand in gs.player_hands.iter_mut() {
            for card_does_not_have in all_must_haves.difference(&player_hand.must_have) {
                if player_hand.must_not_have.insert(card_does_not_have.clone()) {
                    changed = true;
                }
            }
        }

        // Does Not haves -> Must Have
        // If Unknown Accusation, check if the 2 cards they don't have, therefore responding player
        // must have the third card.
        for acc in gs.accusations.iter() {
        
            // Already know the card
            if acc.card_shown.is_some() {
                continue;
            }

            let potentially_shown_cards: HashSet<Card> = 
                vec![
                    Card::SuspectCard(acc.suspect.clone()),
                    Card::RoomCard(acc.room.clone()),
                    Card::WeaponCard(acc.weapon.clone())
                ].into_iter().collect();


            let responding_player_hand = gs.player_hands.get_mut(acc.responding_player_index.unwrap()).unwrap();

            if !potentially_shown_cards.is_disjoint(&responding_player_hand.must_have) {
                // At least one of our potential cards we already know they have, no new info
                continue;
            }

            let potentially_shown_cards: HashSet<&Card> = 
                potentially_shown_cards.difference(&responding_player_hand.must_not_have).collect();

            if potentially_shown_cards.len() == 1 {
                // Only one option for them to show, they must have this card.
                responding_player_hand.must_have.insert(potentially_shown_cards.into_iter().next().unwrap().clone());
                changed = true;
            }
        }


        let number_of_expected_cards_per_hand = gs.get_number_of_expected_cards_per_hand();

        // Must Have -> Does not have
        // If a player's must have is complete, then finish completing their
        // must not have with every other card.
        for player_hand in gs.player_hands.iter_mut() {

            let expected_number_of_does_not_haves_for_complete_hand = 
                Card::get_total_cards() - number_of_expected_cards_per_hand;

            if player_hand.must_not_have.len() == expected_number_of_does_not_haves_for_complete_hand {
                continue;
            }

            if player_hand.must_have.len() == number_of_expected_cards_per_hand {
                let must_not_haves_to_add = &Card::get_all_cards() - &player_hand.must_have;
                player_hand.must_not_have.extend(must_not_haves_to_add);
                changed = true;
            }
        } 
    }

    Ok(())
}

/// Determines what cards must and could be
pub fn get_potentially_winning_cards(gs: &GameState) -> HashSet<Card>{
    let mut potentially_winning_cards: HashSet<Card> = Card::get_all_cards(); 

    // Remove Cards players must have
    for hand in gs.player_hands.iter() {
        potentially_winning_cards = &potentially_winning_cards - &hand.must_have;
    }

    potentially_winning_cards = &potentially_winning_cards - &gs.public_cards;

    potentially_winning_cards
}

pub fn get_guaranteed_winning_cards(gs: &GameState) -> HashSet<Card> {
    // Get all the cards
    let mut common_do_not_haves: HashSet<Card> = Card::get_all_cards();

    // Keep cards that everyone must not have.
    for hand in gs.player_hands.iter() {
        common_do_not_haves.retain(|card| hand.must_not_have.contains(card));
    }

    // Remove elements which are in public cards
    common_do_not_haves = &common_do_not_haves - &gs.public_cards;

    let guaranteed_winning_cards = common_do_not_haves;

    guaranteed_winning_cards
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

    use crate::accusation::Accusation;
    use crate::player_hand::PlayerHand;

    use super::*;

    #[test]
    fn test_propogate_state_not_responding_adds_does_not_haves() {

        let mut player_hands = vec![
            PlayerHand::new("p1".to_owned()),
            PlayerHand::new("p2".to_owned()),
            PlayerHand::new("p3".to_owned()),
            PlayerHand::new("p4".to_owned()),
        ];


        // p1 must have Green
        player_hands
            .get_mut(0)
            .unwrap()
            .must_have
            .insert(Card::SuspectCard(Suspect::Green));
        

        let mut gs = GameState {
            public_cards: HashSet::new(),
            player_hands: player_hands,
            self_index: 0,
            accusations: Vec::new(),
        };

        // Everyone else, not p1, must not have green
        propagate_state(&mut gs).unwrap();


        // Check everyone else must not have green
        for i in vec![1,2,3] {
            assert_eq!(gs.player_hands.get(i).unwrap().must_not_have.len(), 1);
            assert_eq!(gs.player_hands.get(i).unwrap().must_have.len(), 0);

            assert!(gs.player_hands.get(i).unwrap().must_not_have.contains(&Card::SuspectCard(Suspect::Green)));

        }

        // p1 should not change, must have green
        assert_eq!(gs.player_hands.get(0).unwrap().must_not_have.len(), 0);
        assert_eq!(gs.player_hands.get(0).unwrap().must_have.len(), 1);

        assert!(gs.player_hands.get(0).unwrap().must_have.contains(&Card::SuspectCard(Suspect::Green)));
      
        
    }

    #[test]
    fn test_propogate_state_if_player_must_have_other_players_must_not_have() {

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

        propagate_state(&mut gs).unwrap();

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
    fn test_propogate_state_unknown_accusation_with_hand() {

        let mut player_hands = vec![
            PlayerHand::new("p1".to_owned()),
            PlayerHand::new("p2".to_owned()),
            PlayerHand::new("p3".to_owned()),
            PlayerHand::new("p4".to_owned()),
        ];


        // p1 does not have Green or Pistol
        let p1 = player_hands
            .get_mut(0)
            .unwrap();

        p1.must_not_have
            .insert(Card::SuspectCard(Suspect::Green));

        p1.must_not_have
        .insert(Card::WeaponCard(Weapon::Pistol));
        

        let accusations = vec![
            Accusation { 
                accuser_player_index: 3, 
                room: Room::Study, 
                suspect: Suspect::Green, 
                weapon: Weapon::Pistol, 
                responding_player_index: Some(0), 
                card_shown: None
            }
        ];

        let mut gs = GameState {
            public_cards: HashSet::new(),
            player_hands: player_hands,
            self_index: 0,
            accusations: accusations,
        };

        propagate_state(&mut gs).unwrap();


        // Check that p1 must have Study
        assert_eq!(gs.player_hands.get(0).unwrap().must_not_have.len(), 2);
        assert_eq!(gs.player_hands.get(0).unwrap().must_have.len(), 1);

        assert!(gs.player_hands.get(0).unwrap().must_have.contains(&Card::RoomCard(Room::Study)));


        // Check that others must not have Study (since p1 has it)
        for i in vec![1,2,3] {
            assert_eq!(gs.player_hands.get(i).unwrap().must_not_have.len(), 1);
            assert_eq!(gs.player_hands.get(i).unwrap().must_have.len(), 0);

            assert!(gs.player_hands.get(i).unwrap().must_not_have.contains(&Card::RoomCard(Room::Study)));
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