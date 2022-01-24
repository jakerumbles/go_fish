use crate::card::Card;
use crate::deck::*;
use std::fmt;

pub struct Player {
    name: String,
    cards: Deck,
}

impl Player {
    pub fn new(name: &str) -> Player {
        let empty_hand = Deck::new();

        Player {
            name: name.to_string(),
            cards: empty_hand,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "---------------------\nPlayer Name: {}\n{}---------------------\n\n",
            self.name, self.cards
        )
    }
}
