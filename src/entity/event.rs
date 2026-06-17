use std::collections::HashMap;
use derive_builder::Builder;
use eo::sjson::{SJsonElement, SJsonValue};
use crate::hold_builders;

hold_builders!(EntityEvent);

#[derive(Builder, Clone)]
#[builder(setter(into))]
pub struct EntityEvent {
    add: Option<EntityEventComponentGroups>,
    remove: Option<EntityEventComponentGroups>,
    queue_command: Option<EntityEventQueueCommand>,
    randomize: Option<Vec<EntityEvent>>,
    weight: Option<f64>,
    filters: Option<Vec<EntityEventFilter>>,
    sequence: Option<Vec<EntityEvent>>,
    set_property: Option<(String, SJsonValue)>
}

#[derive(Clone)]
pub struct EntityEventComponentGroups {
    component_groups: Vec<String>
}

#[derive(Clone)]
pub struct EntityEventQueueCommand {
    target: String,
    command: Vec<String>
}


#[derive(Clone)]
pub struct EntityEventFilter {
    test: String,
    operator: String,
    value: String
}