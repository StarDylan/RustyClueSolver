use clap::Command;
use cluesolverlib::accusation::Accusation;
use cluesolverlib::player_hand::*;
use std::collections::HashSet;
use std::{iter, vec};
use std::fmt::Display;
use std::str::FromStr;
use std::io;
use error_chain::error_chain;
use colored::*;

use cluesolverlib::solver::*;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

const GAME_STATE_PATH: &str = "game_state.json";

fn main() -> Result<()> {
    let matches = Command::new("cluesolver")
            
        .subcommand(
            Command::new("init")
            .about("Start a new Game"))

        .subcommand(
            Command::new("accuse")
            .about("Add a record of accusation"))
        .subcommand(
            Command::new("verify")
                .about("Verifies Game State")
        )
        .get_matches();


    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            new_game()?;

            Ok(())
        },

        Some(("verify", _sub_matches)) => {
            let gs = match GameState::read_from_file(GAME_STATE_PATH) {
                Ok(game_state) => game_state,

                Err(e) => match e.kind(){
                    io::ErrorKind::NotFound => {
                        eprintln!("File \"{}\" not found!", GAME_STATE_PATH);
                        return Ok(());
                    }

                    _ => {
                    println!("{} {}", "Unable to open file because".red(), e);
                    return Ok(());
                    }
                }
                
            };

            match gs.verify_state() {
                Ok(()) => 
                    println!("{}", "Game State Verified Successfully!".green()),

                Err(reason) => 
                    println!("{} {:?}","Game State Verification failed because".red(), reason)
            }

            Ok(())
        },
         
        Some(("accuse", _sub_matches)) => {
            accuse()?;

            Ok(())
        }
        _ => {
            Ok(())
        }
    }

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

    verify_state_and_save(gs)?;

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


    if !someone_respond {
        responding_player_index = None;
        card_shown = None;
    } else {
        println!("\nWho Responded?");
        responding_player_index = Some(get_player_from_user(&gs.player_hands, vec![accuser_player_index])?);

        if accuser_player_index == gs.self_index {
            println!("\n\nWhat card did they show you?");
            card_shown = Some(get_card_from_user()?);
        } else {

            if gs.self_index == responding_player_index.unwrap() {
                println!("\n\nWhat card did you show them?");
                card_shown = Some(get_card_from_user()?);
            }else {
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
    

    verify_state_and_save(gs)?;

    Ok(())
}

// -------------------------------
// ------User Input Helpers-------
// -------------------------------

pub fn verify_state_and_save(state: GameState) -> Result<()> {
    match state.verify_state() {
        Ok(()) => {
            println!("{} {}", "\nState Verified!".green(), "Saved to file".purple());

            state.save_to_file(GAME_STATE_PATH)?;

            Ok(())
        }

        Err(reason) => {
            println!("{} {:?}", "\nError! Failed to verify init state because of".red(), reason);

            Ok(())
        }
    }

}

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

pub fn get_list_index_from_user<T>(list: &mut dyn Iterator<Item = (usize,T)>) -> Result<usize> where T: Display + Clone {
    
    loop {
        let mut counter = 0;
        let mut indexed_list =  Vec::new();

        for item in &mut *list {
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