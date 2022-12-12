use std::{fmt::{self, write}, collections::HashMap};

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

struct PlayerHand {
    have_suspect: HashMap<Suspect, bool>,
    does_not_have_suspect: HashMap<Suspect, bool>,

    have_weapon: HashMap<Suspect, bool>,
    does_not_have_weapon: HashMap<Suspect, bool>,

    have_room: HashMap<Suspect, bool>,
    does_not_have_room: HashMap<Suspect, bool>,

}