use std::collections::HashMap;
use eo::sjson::SJsonValue;
use crate::core::utilities::Identifier;

pub struct Entity {
    pub id: Identifier,
    pub components: HashMap<String, SJsonValue>,
    // pub events: Vec<EntityEvent>
    // pub component_groups: HashMap<String, EntityComponentGroup>
}