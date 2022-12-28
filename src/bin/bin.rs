use clap::Command;
use cluesolverlib::accusation::Accusation;
use cluesolverlib::cards::*;
use cluesolverlib::player_hand::PlayerHand;
use cluesolverlib::solver::{propagate_state, get_potentially_winning_cards, get_guaranteed_winning_cards};
use std::collections::HashSet;
use std::{iter, vec};
use std::fmt::Display;
use std::str::FromStr;
use std::io;
use colored::*;

use cluesolverlib::errors::*;

use cluesolverlib::game_state::*;


const GAME_STATE_PATH: &str = "game_state.json";

fn main() {
    let matches = Command::new("cluesolver")
            
        .subcommand(
            Command::new("init")
            .about("Start a new Game"))
        .subcommand(
            Command::new("accuse")
            .about("Add a record of accusation"))
        .subcommand(
            Command::new("verify")
                .about("Verifies Game State"))
        .subcommand(
            Command::new("wins")
            .about("Finds potential matches for winning cards"))

        .get_matches();


    let result = match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            new_game()
        },

        Some(("verify", _sub_matches)) => {
            verify()
        },
         
        Some(("accuse", _sub_matches)) => {
            accuse()
        }
        Some(("wins", _sub_matches)) => {
            wins()
        }
        _ => {
            Ok(())
        }
    };

    if let Err(e) = result {
        println!("{} {}", "\nError occurred because".red(), e.to_string());
        ::std::process::exit(1);
    };

}



fn new_game() -> Result<()> {
    println!("Starting a new Game!\n\n");

    let user_name = 
        get_string_from_user("Please Enter your name", |_|{true})?;

    
    let number_of_other_players: usize = 
        get_number_from_user("\nHow many other players are there?")?;

    let mut self_hand: PlayerHand = 
        PlayerHand::new(user_name.trim().to_owned());

    let number_of_public_cards = 18 % (number_of_other_players + 1);

    let number_of_cards_per_player = 
        (18 - number_of_public_cards) / (number_of_other_players + 1);

    println!("\nPlease enter the cards you have:\n");
    
    for _ in 0..number_of_cards_per_player {
        let user_card = get_card_from_user()?;

        self_hand.must_have.insert(user_card);
    }

    println!("Completed with Self Setup...\n");

    let mut other_player_hands: Vec<PlayerHand> = Vec::new();

    for _ in 0..number_of_other_players {
        let other_player_name = 
            get_string_from_user("\n\nPlease Enter the name of the next player:", |_|{true})?;

        other_player_hands.push(PlayerHand::new(other_player_name))
    }

    println!("Who is starting the game?");

    let all_players = 
        iter::once(self_hand.clone())
        .chain(other_player_hands.clone())
        .collect();


    let starting_player = get_player_from_user(&all_players, vec![])?;

    let mut public_cards = HashSet::new();

    for public_card_index in 0..number_of_public_cards {
        println!("\n\nPublic Facing Card #{}", public_card_index);

        let card = get_card_from_user()?;

        public_cards.insert(card);        
    }



    let gs = GameState::new_game_state(self_hand, other_player_hands, starting_player, public_cards);
    
    gs.verify_state()?;

    gs.save_to_file(GAME_STATE_PATH)?;

    println!("{} {}", "Game State Verified Successfully!".green(), "Saved to File.".purple());
    Ok(())

}

fn accuse() -> Result<()> {
    let mut gs = GameState::read_from_file(GAME_STATE_PATH)?;
    
    println!("Who is making the Accusation?");
    let accuser_player_index = get_player_from_user(&gs.player_hands, vec![])?;

    println!("\n{}'s Turn Now!", gs.player_hands[accuser_player_index].player_name.purple());


    println!("\n\nPlease enter their accusation");
    let room = get_room_card_from_user()?;
    println!("");
    let weapon = get_weapon_card_from_user()?;
    println!("");
    let suspect = get_suspect_card_from_user()?;



    let someone_respond = get_yes_no_from_user("\nDid anyone respond? (y/n)")?;

    let responding_player_index: Option<usize>;
    let card_shown: Option<Card>;

    let is_accuser_self = accuser_player_index == gs.self_index;

    if !someone_respond {
        responding_player_index = None;
        card_shown = None;
    } else {
        println!("\nWho Responded?");
        responding_player_index = Some(get_player_from_user(&gs.player_hands, vec![accuser_player_index])?);

        let potential_cards = 
                vec![Card::RoomCard(room.clone()),
                Card::WeaponCard(weapon.clone()),
                Card::SuspectCard(suspect.clone())];

        if is_accuser_self {
            println!("\n\nWhat card did they show you?");

            card_shown = Some(get_list_item_from_user(&mut potential_cards.iter())?.clone());

        } else {

            let is_responder_self = responding_player_index.unwrap() ==  gs.self_index;

            if is_responder_self {
                println!("\n\nWhat card did you show them?");

                card_shown = Some(get_list_item_from_user(&mut potential_cards.iter())?.clone());

            }else {
                // Card shown secretly, no info
                card_shown = None;
            }
        }
    }


    gs.add_accusation(Accusation {
        accuser_player_index,
        room,
        suspect,
        weapon,
        responding_player_index: responding_player_index,
        card_shown: card_shown,
    });

    propagate_state(&mut gs)?;

    gs.verify_state()?;

    gs.save_to_file(GAME_STATE_PATH)?;

    println!("{} {}", "Game State Verified Successfully!".green(), "Saved to File.".purple());

    Ok(())
}

fn verify() -> Result<()> {
    let gs = GameState::read_from_file(GAME_STATE_PATH)?;

    gs.verify_state()?;

    println!("{}", "Game State Verified!".green());

    Ok(())
}

fn wins() -> Result<()> {
    let mut gs = GameState::read_from_file(GAME_STATE_PATH)?;
    
    propagate_state(&mut gs)?;
    
    gs.verify_state()?;


    let guaranteed_wins = get_guaranteed_winning_cards(&gs);
    let mut potential_wins = get_potentially_winning_cards(&gs);

    // Removes any cards that are same type as guaranteed wins
    potential_wins.retain(|card| {
        for gcard in guaranteed_wins.iter() {
            if gcard.variant_eq(card) {
                return false;
            }
        }
        return true;
    });

    
    println!("Guaranteed Win Cards:");
    for card in guaranteed_wins.iter() {
        println!("{}", card);
    }


    println!("\nPotential Win Cards:");
    

    print!("Rooms: ");
    potential_wins.iter().for_each(|card| {
        // Only get Rooms
        if !card.variant_eq(&Card::RoomCard(Room::Hall)) {
            return;
        }
        print!("{}  ", card);
    });

    print!("\nWeapons: ");
    potential_wins.iter().for_each(|card| {
        // Only get Weapons
        if !card.variant_eq(&Card::WeaponCard(Weapon::Pistol)) {
            return;
        }
        print!("{}  ", card);
    });


    print!("\nSuspects: ");
    potential_wins.iter().for_each(|card| {
        // Only get Rooms
        if !card.variant_eq(&Card::SuspectCard(Suspect::Green)) {
            return;
        }
        print!("{}  ", card);
    });

    println!();

    Ok(())
}


// -------------------------------
// ------User Input Helpers-------
// -------------------------------

pub fn get_card_from_user() -> Result<Card> {

    let card_type = get_string_from_user(
        "r) Room\nw) Weapon\ns) Suspect", 
        |user_input| {
            if user_input.chars().nth(0).is_none() {
                return false;
            }

            if user_input.chars().nth(0).unwrap() == 'r' || 
                user_input.chars().nth(0).unwrap() == 'w' || 
                user_input.chars().nth(0).unwrap() == 's' {
                    return true; 
            }

            return false;
        })?;


    loop {
        if card_type.chars().nth(0).unwrap() == 'r' {
            // Room Card
            let selected_room_card = get_room_card_from_user()?;

            return Ok(Card::RoomCard(selected_room_card));
        
        } else if card_type.chars().nth(0).unwrap() == 'w' {
            // Weapon Card
            let selected_room_card = get_weapon_card_from_user()?;

            return Ok(Card::WeaponCard(selected_room_card));

        } else if card_type.chars().nth(0).unwrap() == 's' {
            // Suspect Card
            let selected_room_card = get_suspect_card_from_user()?;

            return Ok(Card::SuspectCard(selected_room_card));
        }
    }
}

pub fn get_room_card_from_user() -> Result<Room> {
    let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Room>())?;

    return Ok(selected_room_card);
}

pub fn get_weapon_card_from_user() -> Result<Weapon> {
    let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Weapon>())?;

    return Ok(selected_room_card);
}

pub fn get_suspect_card_from_user() -> Result<Suspect> {
    let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Suspect>())?;

    return Ok(selected_room_card);
}

pub fn get_string_from_user<F>(prompt: &str, valid_input: F) -> Result<String> where F: Fn(&str) -> bool {
    let stdin = io::stdin();
    let mut user_input = String::new();

    loop {
        println!("{}",prompt);

        stdin.read_line(&mut user_input)?;

        if valid_input(&user_input) {
            return Ok(user_input.trim().to_owned());
        } else {
            println!("Invalid Input, Please try again.");
            user_input = String::new();
        }
    }
}

pub fn get_number_from_user<T: num::Integer + FromStr>(prompt: &str) -> Result<T> {
    let string_from_user 
        = get_string_from_user(prompt, |user_input| {
            user_input.trim().parse::<T>().is_ok()
        })?;

        let number = string_from_user.trim().parse::<T>();

        match number {
            Ok(num) => {
                return Ok(num);
            } 
            Err(_) => {
                panic!("get_number_from_user failed! with {}", string_from_user.trim());
            }
        }
}

pub fn get_player_from_user(players_hands: &Vec<PlayerHand>, exclude_indices: Vec<usize>) -> Result<usize> {
    let mut all_player_iter = 
        players_hands.iter()
            .enumerate()
            .filter_map(|(idx, hand)| {
                if !exclude_indices.contains(&idx) {
                    Some((idx, hand.player_name.clone()))
                } else {
                    None
                }
            });

    let selected_player_index = get_list_index_from_user(&mut all_player_iter)?;
    
    Ok(selected_player_index)
}

pub fn get_yes_no_from_user(prompt: &str) -> Result<bool> {
    let user_response = get_string_from_user(prompt, |user_input| {
        let cleaned_str = user_input.to_lowercase();

        return cleaned_str.trim().starts_with('y') || cleaned_str.trim().starts_with('n');
    })?;

    if user_response.trim().to_lowercase().starts_with('y') {
        return Ok(true);
    } else {
        return Ok(false);
    }
    
}

pub fn get_list_item_from_user<T>(list: &mut dyn Iterator<Item = T>) -> Result<T> where T: Display + Clone {
    
    let collected_list: Vec<T> = list.collect();


    loop {
        let mut counter = 0;

        for item in collected_list.iter() {
            println!("{}) {}", counter, item);
            counter += 1;
        }

        let stdin = io::stdin();
        let mut user_input = String::new();

        stdin.read_line(&mut user_input)?;

        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                if num >= counter {
                    println!("{} is not within the index range!\n", num);
                    continue;
                }

                return Ok(collected_list[num].clone());
            }

            Err(_) => {
                println!("Error parsing \"{}\"\n", user_input.trim());
                continue;
            }
        }
    }
}

pub fn get_list_index_from_user<T>(list_iter: &mut dyn Iterator<Item = (usize,T)>) -> Result<usize> where T: Display + Clone {
    
    let list_as_vec: Vec<(usize, T)> = list_iter.collect();

    loop {
        let list = list_as_vec.iter();

        let mut counter = 0;
        let mut indexed_list =  Vec::new();

        for item in list {
            println!("{}) {}", counter, item.1);
            indexed_list.push(item);
            counter += 1;
        }

        let stdin = io::stdin();
        let mut user_input = String::new();

        stdin.read_line(&mut user_input)?;

        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                if num >= counter {
                    println!("{} is not within the index range!\n", num);
                    continue;
                }

                return Ok(indexed_list[num].0);
            }

            Err(_) => {
                println!("Error parsing \"{}\"\n", user_input.trim());
                continue;
            }
        }
    }
}