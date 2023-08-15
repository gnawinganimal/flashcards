use std::collections::HashMap;

use crate::deck::Deck;

pub struct State {
    decks: HashMap<String, Deck>,
}

impl State {
    pub fn new() -> Self {
        Self {
            decks: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Deck> {
        self.decks.get(name)
    }

    pub fn get_mut(&mut self, name: &String) -> Option<&mut Deck> {
        self.decks.get_mut(name)
    }

    pub fn push(&mut self, deck: Deck) {
        self.decks.insert(deck.name().clone(), deck);
    }
}
