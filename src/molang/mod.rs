use std::ops::{BitAnd, BitOr};
use std::sync::RwLock;
use crate::core::Serializable;

#[derive(Clone, Debug)]
enum MolangConcat {
    And,
    Or,
    None
}

#[derive(Clone, Debug)]
pub struct Molang {
    pub query: String,
    next: Vec<Molang>,
    concat_mode: MolangConcat
}

impl Molang {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            next: Vec::new(),
            concat_mode: MolangConcat::None
        }
    }

    pub fn and(&self, other: Molang) -> Self {
        let mut sc = self.clone();

        sc.concat_mode = MolangConcat::And;
        
        sc.next.push(other);

        sc
    }

    pub fn or(&self, other: Molang) -> Self {
        let mut sc = self.clone();
        
        sc.concat_mode = MolangConcat::Or;

        sc.next.push(other);

        sc
    }
}

impl BitAnd for Molang {
    type Output = Molang;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for Molang {
    type Output = Molang;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl From<String> for Molang {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Molang {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Serializable for Molang {
    fn serialize(&self) -> String {
        let next = self.next.clone();

        let serialized_next = next.into_iter().map(|x| x.serialize()).collect::<Vec<String>>().join(" ");

        let serialized_self = format!("({})", &self.query);

        match &self.concat_mode {
            MolangConcat::And => format!("{} && ({})", serialized_self, serialized_next),
            MolangConcat::Or => format!("{} && ({})", serialized_self, serialized_next),
            MolangConcat::None => self.query.clone(),
        }
    }
}