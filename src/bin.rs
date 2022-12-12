
use clap::{Arg, ArgAction, Command};
use cluesolverlib::hello_world;

fn main() {
    let matches = Command::new("cluesolver")
            
        .subcommand(
            Command::new("init")
                .about("Start a new Game")
        )
        .get_matches();


    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            todo!("init!")
        },
         
        _ => {

        }
    }
    println!("{:?}", matches)
}
