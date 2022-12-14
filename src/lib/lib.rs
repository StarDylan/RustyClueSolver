pub mod solver;
pub mod player_hand;
pub mod user_input;


use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}