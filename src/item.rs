use std::collections::HashMap;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
use crate::core::utilities::Identifier;

pub struct Item {
    pub id: Identifier,
    pub components: HashMap<String, SJsonValue>
}

impl Item {
    pub fn new(id: Identifier, components: Vec<SJsonElement>) -> Self {
        Self {
            id,
            components: components.transform_hashmap()
        }
    }
}