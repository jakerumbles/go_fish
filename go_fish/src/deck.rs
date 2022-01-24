use crate::card::*;
use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::fmt;

/// Length of deck
const DECK_LENGTH: usize = 10;
/// List of card names
const CARD_LIST_A: [&str; DECK_LENGTH] = [
    "Apu",
    "Bogdanoff",
    "Chad",
    "Degen",
    "El Salvador",
    "Maxi",
    "Normie",
    "Pepe",
    "Quantum Immortality Bogdanoff",
    "Wojak",
];

pub struct Deck {
    cards: Vec<Card>,
    length: usize,
}

impl Deck {
    /// create and return a new Deck
    pub fn new() -> Deck {
        let empty_deck: Vec<Card> = vec![];

        println!("Deck created...");

        Deck {
            cards: empty_deck,
            length: 0,
        }
    }

    pub fn init(&mut self) {
        // let mut cards: Vec<Card> = vec![];
        for (i, val) in CARD_LIST_A.into_iter().enumerate() {
            let card = Card::new(i as u8, val.to_string());
            self.cards.push(card);
            self.length += 1;
        }
        println!("Deck initialized...");
    }

    /// Get length of deck
    pub fn length(&self) -> usize {
        self.length
    }

    /// Draw a new card from the deck. This gets the card from the top of the deck and returns an Option<Card>
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        let mut rng = StepRng::new(3, 14);
        let mut irs = Irs::default();

        match irs.shuffle(&mut self.cards, &mut rng) {
            Ok(_) => {
                println!("Deck shuffled!");
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cards = String::new();

        for card in &self.cards {
            let template = format!("\n{}   {}", card.id(), card.name());
            cards.push_str(&template);
        }

        write!(
            f,
            "DECK
------------
ID  Card
--  --------{}\n",
            cards
        )
    }
}
