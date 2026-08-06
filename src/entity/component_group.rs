use std::collections::HashMap;
use derive_builder::Builder;
use eo::sjson::SJsonValue;
use tera::Context;
use crate::bedrock::BedrockSerializable;
use crate::code_gen::TEMPLATES;
use crate::hold_builders;

hold_builders!(EntityComponentGroup);

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct EntityComponentGroup {
    id: String,
    components: HashMap<String, SJsonValue>
}

impl EntityComponentGroup {
    pub fn new(
        id: impl Into<String>,
        components: HashMap<String, SJsonValue>
    ) -> Self {
        Self { id: id.into(), components }
    }
}

impl BedrockSerializable for EntityComponentGroup {
    fn bedrock_serialize(&self) -> String {
        let components = serde_json::to_string(&self.components).unwrap();
        let mut context = Context::new();
        context.insert("id", &self.id);
        context.insert("components", &components);

        TEMPLATES.render("entity/component_group.jsont", &context).unwrap()
    }
}