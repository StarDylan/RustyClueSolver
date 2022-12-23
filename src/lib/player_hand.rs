use std::{fmt::{self}, collections::{HashSet}};

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

#[derive(Debug, Eq, PartialEq, Hash, Clone, enum_iterator::Sequence,serde::Serialize, serde::Deserialize)]
pub enum Suspect {
    Mustard,
    Plum,
    Green,
    Peacock,
    Scarlet,
    White
}

impl fmt::Display for Suspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suspect::Scarlet => write!(f, "Miss Scarlet"),
            Suspect::Green => write!(f, "Mr. Green"),
            Suspect::Mustard => write!(f, "Colonel Mustard"),
            Suspect::Plum => write!(f, "Professor Plum"),
            Suspect::Peacock => write!(f, "Mrs. Peacock"),
            Suspect::White => write!(f, "Mrs. White")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, enum_iterator::Sequence,serde::Serialize, serde::Deserialize)]
pub enum Weapon {
    Rope,
    Candlestick,
    Knife,
    Pipe,
    Pistol,
    Wrench
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Weapon::Rope => write!(f, "Rope"),
            Weapon::Candlestick => write!(f, "Candlestick"),
            Weapon::Knife => write!(f, "Knife"),
            Weapon::Pipe => write!(f, "Lead Pipe"),
            Weapon::Pistol => write!(f, "Pistol"),
            Weapon::Wrench => write!(f, "Wrench")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, enum_iterator::Sequence,serde::Serialize, serde::Deserialize)]
pub enum Room {
    Kitchen,
    Ballroom,
    Conservatory,
    Dining,
    Lounge,
    Hall,
    Study,
    Library,
    Billiard
}


impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Room::Kitchen => write!(f, "Kitchen"),
            Room::Ballroom => write!(f, "Ballroom"),
            Room::Conservatory => write!(f, "Conservatory"),
            Room::Dining => write!(f, "Dining Room"),
            Room::Lounge => write!(f, "Lounge"),
            Room::Hall => write!(f, "Hall"),
            Room::Study => write!(f, "Study"),
            Room::Library => write!(f, "Library"),
            Room::Billiard => write!(f, "Billiard Room"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone,serde::Serialize, serde::Deserialize)]
pub enum Card {
    RoomCard(Room),
    WeaponCard(Weapon),
    SuspectCard(Suspect),
}

impl Card {
    pub fn get_total_cards() -> usize { 
        enum_iterator::cardinality::<Room>() + 
        enum_iterator::cardinality::<Suspect>() +
        enum_iterator::cardinality::<Weapon>() 
        - 3
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Card::RoomCard(room) => write!(f,"{}", room),
            Card::WeaponCard(weapon) => write!(f,"{}", weapon),
            Card::SuspectCard(suspect) => write!(f,"{}", suspect),
        }
    }
}