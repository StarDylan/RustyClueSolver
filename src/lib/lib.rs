pub mod solver;
pub mod player_hand;
pub mod accusation;
pub mod game_state;


pub mod errors {
    use error_chain::error_chain;

    error_chain!{
        errors {
            PlayerMustHaveMoreCardsThenExpected
            PlayerCardContradiction
            PlayerHasPublicCard
            SelfIsNotComplete
            InvalidPlayerIndex
            AccusationContradiction
        }

        foreign_links {
            Io(::std::io::Error);
            ParseInt(::std::num::ParseIntError);
            SerdeSerialization(::serde_json::Error);
        }
    }
}