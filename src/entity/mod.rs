pub mod event;
pub mod component_group;
pub mod registry;
pub mod property;

use std::collections::HashMap;
use std::marker::PhantomData;
use derive_builder::Builder;
use eo::sjson::SJsonValue;
use tera::Context;
use crate::bedrock::BedrockSerializable;
use crate::code_gen::TEMPLATES;
use crate::core::utilities::{Identifier, SemVer};
use crate::entity::component_group::EntityComponentGroup;
use crate::entity::event::EntityEvent;
use crate::entity::property::EntityProperty;
use crate::hold_builders;

hold_builders!(Entity);

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct Entity {
    #[builder(default = "SemVer::latest()")]
    pub format_version: SemVer,
    pub id: Identifier,
    pub components: HashMap<String, SJsonValue>,
    #[builder(default = "Vec::new()")]
    pub events: Vec<EntityEvent>,
    #[builder(default = "Vec::new()")]
    pub properties: Vec<EntityProperty>,
    #[builder(default = "Vec::new()")]
    pub component_groups: Vec<EntityComponentGroup>
}

impl BedrockSerializable for Entity {
    fn bedrock_serialize(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("format_version", &self.format_version.render_dotted());
        ctx.insert("id", &self.id.render());
        ctx.insert("spawn_category", "");
        ctx.insert("is_summonable", &true);
        ctx.insert("is_spawnable", &true);
        ctx.insert("properties", "{}");

        let mut component_groups = "{".to_string();

        for g in &self.component_groups {
            component_groups.push_str(
                &g.bedrock_serialize()
            );
            component_groups.push(',');
        }
        component_groups = component_groups.strip_suffix(',').unwrap().to_string();
        component_groups.push('}');
        ctx.insert("component_groups", &component_groups);
        ctx.insert("components", &serde_json::to_string(&self.components).unwrap());
        ctx.insert("events", &format!("[{}]", &self.events.iter()
            .map(|v| v.bedrock_serialize())
            .collect::<Vec<String>>()
            .join(",")
        ));

        TEMPLATES.render("entity/entity.jsont", &ctx).unwrap()
    }
}