
use clap::{Arg, ArgAction, Command};
use cluesolverlib::player_hand::*;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
    let matches = Command::new("cluesolver")
            
        .subcommand(
            Command::new("init")
                .about("Start a new Game")
        )
        .get_matches();


    match matches.subcommand() {
        Some(("init", sub_matches)) => {

            println!("Starting a new Game!");
            let stdin = io::stdin();

            println!("Please Enter your name");
            let mut user_name = String::new();
            stdin.read_line(&mut user_name)?;


            let mut self_hand: PlayerHand = PlayerHand::new(user_name.trim().to_owned());

            println!("\nPlease enter what cards you have:\n");
            
            
            loop {
                let user_card = get_card();
                match user_card {
                    Ok(card) => {
                        self_hand.must_have.insert(card);
                        println!("Enter next card:\n(Use Ctrl+c when done)")
                    }

                    Err(_) => {
                        break;
                    }
                }
            }

            println!("{:?}",self_hand)


        },
         
        _ => {

        }
    }

    Ok(())
}


fn get_card() -> io::Result<Card> {
    let stdin = io::stdin();


    let mut card_type = String::new();
    loop {
        println!("What type of card? (Or when done, type \"exit\")");
        println!("r) Room");
        println!("w) Weapon");
        println!("s) Suspect");

        stdin.read_line(&mut card_type)?;

        if card_type.trim().contains("exit") {
            return Err(io::Error::from(ErrorKind::Interrupted));
        }

        if card_type.chars().nth(0).is_none() {
            println!("Please try again, no chars found\n");
            continue;
        }

        if card_type.chars().nth(0).unwrap() == 'r' || 
            card_type.chars().nth(0).unwrap() == 'w' || 
            card_type.chars().nth(0).unwrap() == 's' {
            break;
        } else {
            println!("Please try again, invalid char\n")
        }
    }

    println!("\n\nPlease enter the number of the card you have:");

    // TODO: Get rid of all this reptition
    loop {
        let mut card_index = String::new();
        let mut card_index_num = 0;

        if card_type.chars().nth(0).unwrap() == 'r' {
            // Room Card
            let mut counter = 0;

            enum_iterator::all::<Room>().for_each(|room| {
                println!("{}) {}", counter, room);
                counter += 1;
            });

            stdin.read_line(&mut card_index)?;

            match card_index.trim().parse::<usize>() {
                Ok(num) => {
                    if num >= enum_iterator::cardinality::<Room>() {
                        println!("{} is not within the index range!\n", num);
                        continue;
                    }
                    card_index_num = num;
                }

                Err(_) => {
                    println!("Error parsing \"{}\"\n", card_index.trim());
                    continue;
                }
            }
            
            return Ok(Card::RoomCard(enum_iterator::all::<Room>().nth(card_index_num).unwrap()));

        } else if card_type.chars().nth(0).unwrap() == 'w' {
            // Weapon Card
            let mut counter = 0;

            enum_iterator::all::<Weapon>().for_each(|room| {
                println!("{}) {}", counter, room);
                counter += 1;
            });

            stdin.read_line(&mut card_index)?;

            match card_index.trim().parse::<usize>() {
                Ok(num) => {
                    if num >= enum_iterator::cardinality::<Weapon>() {
                        println!("{} is not within the index range!\n", num);
                        continue;
                    }
                    card_index_num = num;
                }

                Err(_) => {
                    println!("Error parsing \"{}\"\n", card_index.trim());
                    continue;
                }
            }
            
            return Ok(Card::WeaponCard(enum_iterator::all::<Weapon>().nth(card_index_num).unwrap()));

        } else if card_type.chars().nth(0).unwrap() == 's' {
            // Suspect Card
            let mut counter = 0;

            enum_iterator::all::<Suspect>().for_each(|room| {
                println!("{}) {}", counter, room);
                counter += 1;
            });

            stdin.read_line(&mut card_index)?;

            match card_index.trim().parse::<usize>() {
                Ok(num) => {
                    if num >= enum_iterator::cardinality::<Suspect>() {
                        println!("{} is not within the index range!\n", num);
                        continue;
                    }
                    card_index_num = num;
                }

                Err(_) => {
                    println!("Error parsing \"{}\"\n", card_index.trim());
                    continue;
                }
            }
            
            return Ok(Card::SuspectCard(enum_iterator::all::<Suspect>().nth(card_index_num).unwrap()));
        }
    }
}