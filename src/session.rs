use std::collections::{btree_map::Iter, HashMap};

use rand::{thread_rng, Rng};
use crate::{deck::Deck, card::Value, schema::Kind};

pub struct Session<'deck> {
    deck:   &'deck Deck,
    weight: HashMap<usize, usize>,
    fst:    String,
    snd:    String,    
}

pub struct Query {
    pub fst: (String, Value),
    pub snd: (String, Value), 
}

impl<'deck> Session<'deck> {
    pub fn new(deck: &'deck Deck, fst: String, snd: String) -> Result<Self, ()> {
        Ok(Self {
            deck,
            weight: HashMap::new(),
            fst,
            snd,
        })
    }

    pub fn weight_of(&self, index: usize) -> Option<usize> {
        if index >= self.deck.len() {
            return None;
        }

        Some(self.weight.get(&index).cloned().unwrap_or(1))
    }

    pub fn bump(&mut self, index: usize) {
        if index >= self.deck.len() {
            return;
        }

        if let Some(weight) = self.weight.get_mut(&index) {
            *weight += 1;
        } else {
            self.weight.insert(index, 2);
        }
    }

    pub fn next(&self) -> Option<Query> {
        let r = thread_rng().gen_range(0..self.deck.len());
        let card = self.deck.get(r)?;

        Some(Query {
            fst: (self.fst.clone(), card.get(&self.fst)?),
            snd: (self.snd.clone(), card.get(&self.snd)?),
        })
    }
}

