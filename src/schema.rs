use std::{collections::HashMap, fmt::Display};

pub struct Schema {
    fields: HashMap<String, Kind>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Text,
    Bool,
    Dec,
    Int,
    Nat,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, field: &str) -> Option<Kind> {
        self.fields.get(&field.to_string()).cloned()
    }

    pub fn set(&mut self, field: &str, kind: Kind) {
        self.fields.insert(field.to_string(), kind);
    }

    pub fn with(mut self, field: &str, kind: Kind) -> Self {
        self.fields.insert(field.to_string(), kind);

        self
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for (field, kind) in &self.fields {
            s += format!("{} :: {}", field, kind).as_str();
        }

        write!(f, "{}", s)
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::Text => "Text",
            Kind::Bool => "Bool",
            Kind::Dec  => "Dec",
            Kind::Int  => "Int",
            Kind::Nat  => "Nat",
        };

        write!(f, "{}", s)
    }
}

impl PartialEq for Schema {
    // one sided, not really equals
    fn eq(&self, other: &Self) -> bool {
        for (field, kind) in &self.fields {
            if matches!(other.get(field), None) {
                return false;
            }

            if other.get(field).unwrap() == *kind {
                return false;
            }
        };

        true
    }
}
