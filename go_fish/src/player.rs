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

    pub fn cards(&mut self) -> &mut Deck {
        &mut self.cards
    }

    /// Draw a card fromm the deck and place in players deck
    pub fn draw_card(&mut self, deck: &mut Deck) {
        match deck.draw() {
            Some(card) => {
                self.cards.add_to_deck(card);
            }
            None => println!("Cannot draw a card. The deck is empty"),
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
