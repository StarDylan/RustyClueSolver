use std::{io::{Write, Read}, fs::File, collections::HashSet};

use error_chain::bail;

use crate::{player_hand::*, accusation::Accusation};

use crate::errors::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameState {
    pub public_cards: HashSet<Card>,

    pub player_hands: Vec<PlayerHand>,
    pub self_index: usize,

    pub accusations: Vec<Accusation>,
}

impl GameState {
    /// Starts a new game
    /// 
    /// Requires your hand to be complete (no unknowns)
    /// Other player names must be in order of play
    /// 
    /// Starting player dictates who is starting, 0 being you, 1 being the first listed
    /// in the other_player_names, and so on.
    pub fn new_game_state(self_hand: PlayerHand, other_player_hands: Vec<PlayerHand>, 
        starting_player: usize, public_cards: HashSet<Card>) -> GameState {

        let mut player_hands: Vec<PlayerHand> = Vec::new();

        let mut self_index: usize = 999;

        // Order the elements correctly other_player_h
        for idx in 0..(other_player_hands.len() + 1) {
            let ordered_index = (starting_player + idx) % (other_player_hands.len() + 1);

            if ordered_index == 0 {
                player_hands.push(self_hand.clone());
                self_index = idx;
            } else {
                player_hands.push(
                    other_player_hands.get(ordered_index - 1).unwrap().to_owned()
                );
            }
        }

        GameState {
            public_cards,

            player_hands,
            self_index,

            accusations: Vec::new(),
        }

        
    }


    /// Ensures the state makes sense
    /// 
    /// All of players' must haves must be less than the number
    /// of cards we expect based on player count. Vice-versa with
    /// must not haves.
    /// 
    /// There must be no contradictions between must have and must not have.
    /// 
    /// Self must be complete
    pub fn verify_state(&self) -> Result<()> {

        // Check all player indecies are valid
        if self.self_index >= self.player_hands.len() {
            bail!(ErrorKind::InvalidPlayerIndex(
                "self index".to_owned(),
                self.self_index
            ))
        }


        // If a card is shown, then the responding player must be Some()
        for acc in self.accusations.iter() {
            if acc.card_shown.is_some() && acc.responding_player_index.is_none() {
                bail!(ErrorKind::AccusationContradiction)
            }   
        }

        for acc in self.accusations.iter() {

            if acc.accuser_player_index >= self.player_hands.len() {
                bail!(ErrorKind::InvalidPlayerIndex(
                    "accusation accuser player index".to_owned(),
                    acc.accuser_player_index
                ))
            }

            if acc.responding_player_index.is_none() {
                continue;
            }

            if acc.responding_player_index.unwrap() >= self.player_hands.len() {
                bail!(ErrorKind::InvalidPlayerIndex(
                    "accusation responding player index".to_owned(),
                    acc.responding_player_index.unwrap()
                ))
            }
        }


        let number_of_cards_expected = self.get_number_of_expected_cards_per_hand();

        let mut already_must_have_cards: HashSet<Card> = HashSet::new();

        for player in self.player_hands.iter() {

            if !already_must_have_cards.is_disjoint(&player.must_have) {
                // Overlapping, Contradiction since two different players can't
                // have the same card.
                let intersection = already_must_have_cards.intersection(&player.must_have);
                bail!(ErrorKind::PlayerCardContradiction(
                    player.player_name.to_owned(),
                    intersection.into_iter().next().unwrap().clone(),
                    "player must have the same card as another player".to_owned()
                ));
            }

            // Update the cards we've already checked against
            for card in &player.must_have {
                already_must_have_cards.insert(card.clone());
            }

            if player.must_have.len() > number_of_cards_expected {
                bail!(ErrorKind::PlayerHasInvalidCardNumber(
                    player.player_name.clone(),
                    player.must_have.len(),
                    "Must Have Cards".to_owned(), 
                    number_of_cards_expected)
                );
            }

            if !player.must_have.is_disjoint(&player.must_not_have) {
                // Elements in common, thus contradiction

                let intersection = player.must_have.intersection(&player.must_not_have);

                bail!(ErrorKind::PlayerCardContradiction(
                    player.player_name.to_owned(),
                    intersection.into_iter().next().unwrap().clone(),
                    "player has card in both must have and must not have".to_owned()
                ));
            }

            if player.must_not_have.len() > (Card::get_total_cards() - number_of_cards_expected) {
                // Can't have more cards then possible
                bail!(ErrorKind::PlayerHasInvalidCardNumber(
                    player.player_name.clone(),
                    player.must_not_have.len(),
                    "Must not Have Cards".to_owned(), 
                    Card::get_total_cards() - number_of_cards_expected)
                );
            }

            if !self.public_cards.is_disjoint(&player.must_have) {
                // Can't have any of the public cards
                // Displays first public card found
                let public_card_intersection = self.public_cards.intersection(&player.must_have);
                bail!(ErrorKind::PlayerHasPublicCard(
                    player.player_name.clone(), 
                    public_card_intersection.into_iter().next().unwrap().clone()));
            }
        }

        let self_hand = &self.player_hands[self.self_index];

        // If someone has a card they showed, then that must not be in their must not have.
        for accusation in &self.accusations {
            if accusation.card_shown.is_some() {
                let responding_player = 
                    self.player_hands.get(accusation.responding_player_index.unwrap()).unwrap();

                if responding_player.must_not_have.contains(&accusation.card_shown.as_ref().unwrap()) {

                    let card = responding_player
                        .must_not_have.iter()
                        .find(|card| card.clone() == accusation.card_shown.as_ref().unwrap())
                        .unwrap();

                    bail!(ErrorKind::PlayerCardContradiction(
                        responding_player.player_name.to_owned(),
                        card.clone(),
                        "player has showed a card they must not have".to_owned()
                    ));
                }
            }
        }

        if !self_hand.is_complete(number_of_cards_expected) {
            bail!(ErrorKind::SelfIsNotComplete);
        }

        return Ok(());
    }

    pub fn add_accusation(&mut self, accusation: Accusation) {
        self.accusations.push(accusation);
    }

    pub fn get_number_of_expected_cards_per_hand(&self) -> usize {
        let number_of_cards_not_in_center = Card::get_total_cards() - 3;

        (number_of_cards_not_in_center - (number_of_cards_not_in_center % self.player_hands.len())) / self.player_hands.len()
    }


    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let serialized = serde_json::to_string_pretty(self)?;            
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        file.flush()?;

        Ok(())
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut json_buf: String = String::new();
        file.read_to_string(&mut json_buf)?;
        let deserialized: GameState = serde_json::from_str(&json_buf)?;

        Ok(deserialized)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_game_state_self_start() {
        let p1 = PlayerHand::new("Dylan".to_owned());
        let p2 = PlayerHand::new("Alice".to_owned());
        let p3 = PlayerHand::new("Bob".to_owned());

        let other_players = vec![p2.clone(), p3.clone()];

        let gs = GameState::new_game_state(p1.clone(), other_players, 0,
            HashSet::new());

        for it in gs.player_hands.iter().zip(vec![p1,p2,p3].iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }

        assert_eq!(gs.self_index, 0)
    }

    #[test]
    fn test_new_game_state_middle_start() {
        let p1 = PlayerHand::new("Dylan".to_owned());
        let p2 = PlayerHand::new("Alice".to_owned());
        let p3 = PlayerHand::new("Bob".to_owned());
        let p4 = PlayerHand::new("Rob".to_owned());

        let other_players = vec![p2.clone(), p3.clone(), p4.clone()];

        let gs = GameState::new_game_state(p1.clone(), other_players, 2,
            HashSet::new());

        for it in gs.player_hands.iter().zip(vec![p3,p4,p1,p2].iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }

        assert_eq!(gs.self_index, 2)
    }

    #[test]
    fn test_new_game_state_last_start() {
        let p1 = PlayerHand::new("Dylan".to_owned());
        let p2 = PlayerHand::new("Alice".to_owned());
        let p3 = PlayerHand::new("Bob".to_owned());
        let p4 = PlayerHand::new("Rob".to_owned());

        let other_players = vec![p2.clone(), p3.clone(), p4.clone()];

        let gs = GameState::new_game_state(p1.clone(), other_players, 3,
            HashSet::new());

        for it in gs.player_hands.iter().zip(vec![p4, p1, p2, p3].iter()) {
            let (e1, e2) = it;
            assert_eq!(*e1, *e2);
        }


        assert_eq!(gs.self_index, 1)
    }
    
}