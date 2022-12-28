use std::collections::HashSet;

use crate::cards::Card;

/// Contains details about what we know / don't know about a 
/// player's hand.
/// 
/// For example, we might know they have a certain suspect card,
/// so we mark it under the have_suspect field. Same with weapon and
/// room.
/// 
/// String contains a nice user-readable name to differentiate between
/// hands.

// Callers from outside my crate can't directly construct me
// or exhaustively match on my fields!
#[non_exhaustive]
#[derive(Debug, Clone,serde::Serialize, serde::Deserialize, PartialEq)]
pub struct PlayerHand {
    pub player_name: String,

    pub must_have: HashSet<Card>,
    pub must_not_have: HashSet<Card>
}

impl PlayerHand {
    pub fn new(player_name: String) -> PlayerHand {
        PlayerHand { 
            player_name, 

            must_have: HashSet::new(), 
            must_not_have: HashSet::new(),
        }
    }

    pub fn is_complete(&self, expected_number_of_cards: usize) -> bool {
        if self.must_have.len() == expected_number_of_cards {
            return true;
        }

        if self.must_not_have.len() == (18 - expected_number_of_cards - self.must_have.len()) {
            return true;
        }
        
        return false;
    }
}