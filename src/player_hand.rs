use std::{fmt::{self}, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug, Eq, PartialEq, Hash)]
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

/// Contains details about what we know / don't know about a 
/// player's hand.
/// 
/// For example, we might know they have a certain suspect card,
/// so we mark it under the have_suspect field. Same with weapon and
/// room.
/// 
/// String contains a nice user-readable name to differentiate between
/// hands.
#[derive(Debug)]
pub struct PlayerHand {
    pub player_name: String,

    pub have_suspect: HashMap<Suspect, bool>,
    pub does_not_have_suspect: HashMap<Suspect, bool>,

    pub have_weapon: HashMap<Suspect, bool>,
    pub does_not_have_weapon: HashMap<Suspect, bool>,

    pub have_room: HashMap<Suspect, bool>,
    pub does_not_have_room: HashMap<Suspect, bool>,
}

impl PlayerHand {
    pub fn new(name: String) -> PlayerHand {
        PlayerHand { 
            player_name: name, 

            have_suspect: HashMap::new(), 
            does_not_have_suspect: HashMap::new(),

            have_weapon: HashMap::new(), 
            does_not_have_weapon: HashMap::new(), 
            have_room: HashMap::new(), 
            does_not_have_room: HashMap::new() 
        }
    }
}