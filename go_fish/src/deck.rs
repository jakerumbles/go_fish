use crate::card::*;
use crate::player::*;
use rand::rngs::mock::StepRng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;
use std::fmt;

/// Length of deck
const DECK_LENGTH: usize = 24;
/// List of card names
const CARD_LIST_A: [&str; DECK_LENGTH] = [
    "Apu",
    "Apu",
    "Bogdanoff",
    "Bogdanoff",
    "Chad",
    "Chad",
    "Degen",
    "Degen",
    "El Salvador",
    "El Salvador",
    "Gigachad",
    "Gigachad",
    "Just Jerome",
    "Just Jerome",
    "Maxi",
    "Maxi",
    "Normie",
    "Normie",
    "Pepe",
    "Pepe",
    "Quantum Immortality Bogdanoff",
    "Quantum Immortality Bogdanoff",
    "Wojak",
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

    /// Shuffle the cards vector in-place
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

    /// Deal 7 cards to each player at the beginning of the game
    pub fn deal_cards(&mut self, p1: &mut Player, p2: &mut Player) {
        for _ in 0..7 {
            println!("LOOPING...");
            self.card_to_player(p1);
            self.card_to_player(p2);
        }
    }

    /// Deal 1 card to player per method call at the beginning of the game
    fn card_to_player(&mut self, player: &mut Player) {
        match self.draw() {
            Some(card) => {
                player.cards().add_to_deck(card);
            }
            None => println!("Cannot draw a card. The deck is empty"),
        }
    }

    /// Add a new card to top of deck
    pub fn add_to_deck(&mut self, card: Card) {
        self.cards.push(card);
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
