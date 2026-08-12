use std::collections::HashMap;
use std::fmt::Debug;
use std::range::Range;
use std::rc::Rc;
use std::sync::Arc;
use derive_builder::Builder;
use crate::core::utilities::Identifier;
use crate::hold_builders;

hold_builders!(EntityProperty, EntityFloatProperty, EntityIntProperty, EntityEnumProperty);

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityProperty {
    id: Identifier,
    client_sync: bool,
    property: EntityPropertyType
}

#[derive(Clone, Debug)]
pub enum EntityPropertyType {
    Enum(EntityEnumProperty),
    Int(EntityIntProperty),
    Float(EntityFloatProperty)
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityFloatProperty {
    range: Range<f32>,
    default: f32
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityIntProperty {
    range: Range<i32>,
    default: i32
}

#[derive(Clone, Debug, Builder)]
pub struct EntityEnumProperty {
    #[builder(setter(into))]
    default: String,
    values: Vec<String>
}

impl From<EntityIntProperty> for EntityPropertyType {
    fn from(value: EntityIntProperty) -> Self {
        EntityPropertyType::Int(value)
    }
}

impl From<EntityFloatProperty> for EntityPropertyType {
    fn from(value: EntityFloatProperty) -> Self {
        EntityPropertyType::Float(value)
    }
}

impl From<EntityEnumProperty> for EntityPropertyType {
    fn from(value: EntityEnumProperty) -> Self {
        EntityPropertyType::Enum(value)
    }
}