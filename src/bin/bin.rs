use clap::Command;
use cluesolverlib::player_hand::PlayerHand;
use std::vec;

use cluesolverlib::solver::*;
use cluesolverlib::user_input::*;


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

    let mut self_hand: PlayerHand = PlayerHand::new(user_name.trim().to_owned(), number_of_cards);


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