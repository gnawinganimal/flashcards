use std::collections::HashMap;

use crate::schema::{Schema, Kind};

pub struct Card {
    fields: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Text(String),
    Bool(bool),
    Dec(u64),
    Int(i64),
    Nat(f64),
}

impl Card {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, field: &str) -> Option<Value> {
        self.fields.get(&field.to_string()).cloned()
    }

    pub fn set(&mut self, field: &str, value: Value) {
        self.fields.insert(field.to_string(), value);
    }

    pub fn has(&self, field: &str) -> bool {
        self.fields.contains_key(&field.to_string())
    }

    pub fn kind_of(&self, field: &str) -> Option<Kind> {
        self.fields.get(field).map(|value| value.kind_of())
    }

    pub fn schema_of(&self) -> Schema {
        let mut schema = Schema::new();

        for (field, value) in &self.fields {
            schema.set(field.as_str(), value.kind_of());
        }

        schema
    }
}

impl Value {
    pub fn kind_of(&self) -> Kind {
        match self {
            Self::Text(_) => Kind::Text,
            Self::Bool(_) => Kind::Bool,
            Self::Dec(_)  => Kind::Dec,
            Self::Int(_)  => Kind::Int,
            Self::Nat(_)  => Kind::Nat,
        }
    } 
}
