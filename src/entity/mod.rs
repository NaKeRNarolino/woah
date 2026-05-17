mod event;
mod component_group;

use std::collections::HashMap;
use eo::sjson::SJsonValue;
use crate::core::utilities::Identifier;
use crate::entity::component_group::EntityComponentGroup;
use crate::entity::event::EntityEvent;

pub struct Entity {
    pub id: Identifier,
    pub components: HashMap<String, SJsonValue>,
    pub events: Vec<EntityEvent>,
    pub component_groups: Vec<EntityComponentGroup>
}