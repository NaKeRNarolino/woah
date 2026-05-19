use crate::code_gen::TEMPLATES;
use crate::core::utilities::{Identifier, SemVer};
use crate::bedrock::BedrockSerializable;
use eo::sjson::{SJsonElement, SJsonValue, TransformHashMap};
use std::collections::HashMap;
use derive_builder::Builder;
use crate::hold_builders;

pub mod item_registry;
pub mod client;

hold_builders!(Item);

/// A struct for describing Items. Use [eo::sjson!] for components.
#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Item {
    pub id: Identifier,
    #[builder(default = "SemVer::latest()")]
    pub format_version: SemVer,
    pub components: HashMap<String, SJsonValue>
}



impl BedrockSerializable for Item {
    fn bedrock_serialize(&self) -> String {
        let components_serialized = serde_json::to_string(&self.components).unwrap();
        
        let mut c = tera::Context::new();
        
        c.insert("format_version", &self.format_version.render_dotted());
        c.insert("components", &components_serialized);
        c.insert("id", &self.id.render());
        
        TEMPLATES.render("items/item.json", &c).unwrap()
    }
}