use go_fish::card::Card;

let card_list = vec![
    "Autist",
    "Bro",
    "Chad",
    "Degen",
    "Pepe",
    "Wojak",
];

pub struct Deck {
    cards: Vec<Card>
};

impl Deck {
    pub fn new() -> Deck {
        let cards: Vec<Card> = vec![];
        for (i, val) in card_list.into_iter().enumerate() {
            let card = Card {
                id: i,
                name: val,
            };

            cards.push(card);
        }

        Deck {
            cards
        }
    }
}
