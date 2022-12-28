pub mod solver;
pub mod player_hand;
pub mod accusation;
pub mod game_state;
pub mod cards;


pub mod errors {
    use error_chain::error_chain;

    use crate::cards::Card;

    error_chain!{
        errors {
            PlayerHasInvalidCardNumber(player: String, actual: usize, list_type: String, limit: usize) {
                description("Player has an invalid number of cards")
                display("\"{}\" has {} {}, while the limit is {}", player, actual, list_type, limit)
            }
            PlayerCardContradiction(player: String, card: Card, reason: String) {
                description("Player is contradictory to other info")
                display("\"{}\" has contradictory card \"{}\" because {}", player, card, reason)
            }
            PlayerHasPublicCard(player: String, cards: Card) {
                description("Player must have a publicly shown card")
                display("\"{}\" has public card \"{}\"", player, cards)
            }
            SelfIsNotComplete {
                description("Player Self does not have complete set of cards")
                display("Self Player does not have complete set of cards")
                
            }
            InvalidPlayerIndex(location: String, actual: usize) {
                description("Invalid Player Index")
                display("invalid index of {} at {}",actual, location)
            }
            AccusationContradiction {
                description("Accusation card is shown, yet responding player is none")
                display("accusation card is shown, yet responding player is none")
            }
        }

        foreign_links {
            Io(::std::io::Error);
            ParseInt(::std::num::ParseIntError);
            SerdeSerialization(::serde_json::Error);
        }
    }
}