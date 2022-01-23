use std::fmt;

pub struct Card {
    id: u8,
    name: String,
}

impl Card {
    pub fn new(id: u8, name: String) -> Card {
        Card { id, name }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card\n|  Card ID: {}\n|  Name: {}", self.id, self.name())
    }
}

pub fn works() {
    println!("hellurrrrr");
}
