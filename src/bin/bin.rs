use clap::Command;
use cluesolverlib::player_hand::*;
use std::vec;
use std::fmt::Display;
use std::str::FromStr;
use std::io;
use error_chain::error_chain;

use cluesolverlib::solver::*;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn main() -> Result<()> {
    let matches = Command::new("cluesolver")
            
        .subcommand(
            Command::new("init")
                .about("Start a new Game")
        )
        .get_matches();


    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            new_game()?;

            Ok(())
        },
         
        _ => {
            Ok(())
        }
    }

}



fn new_game() -> Result<()> {
    println!("Starting a new Game!\n\n");

    let user_name = 
        get_string_from_user("Please Enter your name", |_|{true})?;

    
    let number_of_cards: usize = 
        get_number_from_user("\n\nHow many cards do you have?")?;

    let mut self_hand: PlayerHand = 
        PlayerHand::new(user_name.trim().to_owned(), number_of_cards);


    println!("\nPlease enter what cards you have:\n");
    
    for _ in 0..number_of_cards {
        let user_card = get_card_from_user();
        match user_card {
            Ok(card) => {
                self_hand.must_have.insert(card);
                println!("Enter next card:\n")
            }

            Err(e) => {
                return Err(e);
            }
        }
    }


    let state = GameState {
        player_hands: vec![self_hand]
    };

    println!("{:#?}", state);

    //state.save_to_file("game_state.json");

    let deserialized_state = GameState::read_from_file("game_state.json").unwrap();
    println!("{:?}", deserialized_state);
    
    Ok(())
}


// -------------------------------
// ------User Input Helpers-------
// -------------------------------

pub fn get_card_from_user() -> Result<Card> {

    let card_type = get_string_from_user(
        "What type of card?\nr) Room\nw) Weapon\ns) Suspect", 
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


    println!("\n\nPlease enter the number of the card you have:");

    loop {
        if card_type.chars().nth(0).unwrap() == 'r' {
            // Room Card
            let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Room>())?;

            return Ok(Card::RoomCard(selected_room_card));

        } else if card_type.chars().nth(0).unwrap() == 'w' {
            // Weapon Card
            let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Weapon>())?;

            return Ok(Card::WeaponCard(selected_room_card));

        } else if card_type.chars().nth(0).unwrap() == 's' {
            // Suspect Card
            let selected_room_card = get_list_item_from_user(&mut enum_iterator::all::<Suspect>())?;

            return Ok(Card::SuspectCard(selected_room_card));
        }
    }
}


pub fn get_string_from_user<F>(prompt: &str, valid_input: F) -> Result<String> where F: Fn(&str) -> bool {
    let stdin = io::stdin();
    let mut user_input = String::new();

    loop {
        println!("{}",prompt);

        stdin.read_line(&mut user_input)?;

        if valid_input(&user_input) {
            return Ok(user_input);
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

pub fn get_list_item_from_user<T>(list: &mut dyn Iterator<Item = T>) -> Result<T> where T: Display + Clone {
    
    loop {
        let mut counter = 0;
        let mut indexed_list =  Vec::new();

        for item in &mut *list {
            println!("{}) {}", counter, item);
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

                return Ok(indexed_list[num].clone());
            }

            Err(_) => {
                println!("Error parsing \"{}\"\n", user_input.trim());
                continue;
            }
        }
    }
}