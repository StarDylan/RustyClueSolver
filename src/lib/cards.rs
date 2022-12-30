use std::{collections::HashSet, fmt};

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
    }

    pub fn variant_eq(&self, b: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }

    pub fn get_all_cards() -> HashSet<Card> {
        let all_cards = 
            enum_iterator::all::<Room>()
                .map(Card::RoomCard)
            .chain(enum_iterator::all::<Suspect>()
                .map(Card::SuspectCard))
            .chain(enum_iterator::all::<Weapon>()
                .map(Card::WeaponCard));

        all_cards.collect()
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