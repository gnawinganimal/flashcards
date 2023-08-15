use crate::{card::Card, schema::{Schema, Kind}};

pub struct Deck {
    name:   String,
    schema: Schema,
    cards:  Vec<Card>,
}

pub enum Error {
    SchemaError,
}

impl Deck {
    pub fn new(name: String, schema: Schema) -> Deck {
        Self {
            name,
            schema,
            cards: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn get(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }

    pub fn kind_of(&self, field: &String) -> Option<Kind> {
        self.schema.get(field)
    }

    pub fn push(&mut self, card: Card) -> Result<(), Error> {
        if self.schema != card.schema_of() {
            return Err(Error::SchemaError);
        }

        self.cards.push(card);

        Ok(())
    }
}
