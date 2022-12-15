use std::{io::{self, Write, Read}, fs::File, collections::HashSet};

use crate::player_hand::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameState {
    pub public_cards: HashSet<Card>,

    pub player_hands: Vec<PlayerHand>,
    pub self_index: usize,
}


#[derive(Debug)]
pub enum GameStateVerifyError {
    PlayerMustHaveMoreCardsThenExpected,
    PlayerCardContradiction,
    PlayerHasPublicCard,
    SelfIsNotComplete

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

        // Order the elements correctly
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
            self_index
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
    pub fn verify_state(&self) -> Result<(), GameStateVerifyError> {
        //todo!("Do a result with Errors");

        let number_of_players = self.player_hands.len();
        let public_cards = 18 % number_of_players;
        let number_of_cards_expected = (18 - public_cards) / number_of_players;


        let mut already_must_have_cards: HashSet<Card> = HashSet::new();

        for player in self.player_hands.iter() {

            if !already_must_have_cards.is_disjoint(&player.must_have) {
                // Overlapping, Contradiction since two different players can't
                // have the same card.
                return Err(GameStateVerifyError::PlayerCardContradiction);
            }

            // Update the cards we've already checked against
            for card in &player.must_have {
                already_must_have_cards.insert(card.clone());
            }

            if player.must_have.len() > number_of_cards_expected {
                return Err(GameStateVerifyError::PlayerMustHaveMoreCardsThenExpected);
            }

            if !player.must_have.is_disjoint(&player.must_not_have) {
                // Elements in common, thus contradiction
                return Err(GameStateVerifyError::PlayerCardContradiction);
            }

            if player.must_not_have.len() > (18 - number_of_cards_expected) {
                // Can't have less cards then required to
                return Err(GameStateVerifyError::PlayerMustHaveMoreCardsThenExpected);
            }

            if !self.public_cards.is_disjoint(&player.must_have) {
                // Can't have any of the public cards
                return Err(GameStateVerifyError::PlayerHasPublicCard);
            }
        }

        let self_hand = &self.player_hands[self.self_index];

        if !self_hand.is_complete(number_of_cards_expected) {
            return Err(GameStateVerifyError::SelfIsNotComplete);
        }

        return Ok(());
    }

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