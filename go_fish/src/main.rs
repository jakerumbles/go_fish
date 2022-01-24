mod card;
mod deck;
mod player;

use deck::*;
use player::*;

fn main() {
    println!("Welcome to Go Fish!");
    println!("Starting game...\n");

    let p1 = Player::new("Jake");
    let p2 = Player::new("day1_degen");

    println!("{}", p1);
    println!("{}", p2);

    let mut deck = Deck::new();
    deck.init();
    deck.shuffle();
    println!("{}\n", deck);

    match deck.draw() {
        Some(card) => {
            println!("Drawing card...");
            println!("{}", card);
        }
        None => {
            println!("The draw deck is empty. Cannot draw a card...");
        }
    };
}
