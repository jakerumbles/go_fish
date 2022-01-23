use go_fish::card;
use go_fish::card::Card;

use go_fish::deck;

fn main() {
    println!("Welcome to Go Fish!");
    println!("Starting game...");
    let card = Card::new(12, "pepe".to_string());

    println!("{}", card);

    let deck = deck::Deck::new();
}
