use crate::cards::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Accusation {
    pub accuser_player_index: usize,
    
    pub room: Room,
    pub suspect: Suspect,
    pub weapon: Weapon,

    pub responding_player_index: Option<usize>,

    /// Only if the user has seen the card.
    pub card_shown: Option<Card>,
}
