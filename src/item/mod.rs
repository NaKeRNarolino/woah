use crate::code_gen::TEMPLATES;
use crate::core::utilities::{Identifier, SemVer};
use crate::core::Serializable;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
use std::collections::HashMap;

pub mod item_registry;
pub mod client;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: Identifier,
    pub format_version: SemVer,
    pub components: HashMap<String, SJsonValue>
}

impl Item {
    pub fn new(id: Identifier, components: Vec<SJsonElement>) -> Self {
        Self {
            id,
            components: components.transform_hashmap(),
            format_version: SemVer::latest()
        }
    }

    pub fn using_format_version(&self, format_version: SemVer) -> Self {
        Self {
            format_version,
            ..self.clone()
        }
    }
}

impl Serializable for Item {
    fn serialize(&self) -> String {
        let components_serialized = serde_json::to_string(&self.components).unwrap();
        
        let mut c = tera::Context::new();
        
        c.insert("format_version", &self.format_version.render_dotted());
        c.insert("components", &components_serialized);
        c.insert("id", &self.id.render());
        
        TEMPLATES.render("items/item.json", &c).unwrap()
    }
}