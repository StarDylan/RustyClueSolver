use clap::Command;
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
        Some(("init", _sub_matches)) => {

            println!("Starting a new Game!\n\n");
            let stdin = io::stdin();

            println!("Please Enter your name");
            let mut user_name = String::new();
            stdin.read_line(&mut user_name)?;



            println!("\n\nHow many cards do you have?");
            
            let number_of_cards: usize;
            loop {
                let mut number_of_cards_str = String::new();
                stdin.read_line(&mut number_of_cards_str)?;

                match number_of_cards_str.trim().parse::<usize>() {
                    Ok(num) => {
                        number_of_cards = num;
                        break;
                    }

                    Err(_) => {
                        println!("Error parsing \"{}\"", number_of_cards_str.trim());
                        continue;
                    }
                }
            }

            let mut self_hand: PlayerHand = PlayerHand::new(user_name.trim().to_owned(), number_of_cards);

            println!("\nPlease enter what cards you have:\n");
            
            
            for _ in 0..number_of_cards {
                let user_card = get_card();
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

            println!("{:?}",self_hand);

            Ok(())
        },
         
        _ => {
            Ok(())
        }
    }

}


fn get_card() -> io::Result<Card> {
    let stdin = io::stdin();


    let mut card_type = String::new();
    loop {
        println!("What type of card?");
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
        let card_index_num: usize;

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