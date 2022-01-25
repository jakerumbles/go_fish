mod card;
mod deck;
mod player;

use deck::*;
use player::*;

fn main() {
    println!("Welcome to Go Fish!");
    println!("Starting game...\n");

    let mut p1 = Player::new("Jake");
    let mut p2 = Player::new("day1_degen");

    println!("{}", p1);
    println!("{}", p2);

    let mut deck = Deck::new();
    deck.init();
    deck.shuffle();
    println!("{}\n", deck);

    // Deal the cards
    deck.deal_cards(&mut p1, &mut p2);
    println!("{}", p1);
    println!("{}", p2);

    // Players are ready to begin game...

    // match deck.draw() {
    //     Some(card) => {
    //         println!("Drawing card...");
    //         println!("{}", card);
    //     }
    //     None => {
    //         println!("The draw deck is empty. Cannot draw a card...");
    //     }
    // };
}
