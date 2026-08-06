use std::collections::HashMap;
use derive_builder::Builder;
use eo::sjson::{SJsonElement, SJsonMacro, SJsonValue};
use tera::Context;
use crate::bedrock::BedrockSerializable;
use crate::code_gen::TEMPLATES;
use crate::hold_builders;

hold_builders!(EntityEvent, EntityEventComponentGroups, EntityEventQueueCommand, EntityFilter);

#[derive(Builder, Clone, Debug)]
#[builder(setter(into))]
pub struct EntityEvent {
    #[builder(default = "None")]
    add: Option<EntityEventComponentGroups>,
    #[builder(default = "None")]
    remove: Option<EntityEventComponentGroups>,
    #[builder(default = "None")]
    queue_command: Option<EntityEventQueueCommand>,
    #[builder(default = "None")]
    randomize: Option<Vec<EntityEvent>>,
    #[builder(default = "None")]
    weight: Option<f64>,
    #[builder(default = "None")]
    filters: Option<Vec<EntityFilter>>,
    #[builder(default = "None")]
    sequence: Option<Vec<EntityEvent>>,
    #[builder(default = "None")]
    set_property: Option<SJsonMacro>
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityEventComponentGroups {
    component_groups: Vec<String>
}

#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityEventQueueCommand {
    target: String,
    commands: Vec<String>
}

impl EntityEventQueueCommandBuilder {
    pub fn command(&mut self, command: impl Into<String>) -> &mut Self {
        if self.commands.is_none() { self.commands = Some(Vec::new()) }
        self.commands.as_mut().map(|v| v.push(command.into()));
        self
    }
}


#[derive(Clone, Debug, Builder)]
#[builder(setter(into))]
pub struct EntityFilter {
    test: String,
    operator: String,
    value: String
}

impl BedrockSerializable for EntityFilter
{
    fn bedrock_serialize(&self) -> String {
        let mut ctx = Context::new();

        ctx.insert("test", &self.test);
        ctx.insert("operator", &self.operator);
        ctx.insert("value", &format!("{:?}", &self.value));

        TEMPLATES.render("entity/events/filter.jsont", &ctx).unwrap()
    }
}

impl BedrockSerializable for EntityEvent {
    fn bedrock_serialize(&self) -> String {
        let mut res = "{".to_string();

        if let Some(add) = &self.add {
            let mut ctx = Context::new();
            ctx.insert("component_groups", &format!("{:?}", &add.component_groups));
            res.push_str(
                &TEMPLATES.render("entity/events/add.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(remove) = &self.remove {
            let mut ctx = Context::new();
            ctx.insert("component_groups", &format!("{:?}", &remove.component_groups));
            res.push_str(
                &TEMPLATES.render("entity/events/remove.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(cmd) = &self.queue_command {
            let mut ctx = Context::new();
            ctx.insert("commands", &format!("{:?}", &cmd.commands));
            ctx.insert("target", &cmd.target);
            res.push_str(
                &TEMPLATES.render("entity/events/queue_command.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(rnd) = &self.randomize {
            let mut ctx = Context::new();
            ctx.insert("other", &rnd.iter().map(|v| v.bedrock_serialize()).collect::<Vec<String>>().join(","));

            res.push_str(
                &TEMPLATES.render("entity/events/randomize.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(w) = &self.weight {
            let mut ctx = Context::new();
            ctx.insert("k", "weight");
            ctx.insert("v", w);

            res.push_str(
                &TEMPLATES.render("generic/kv.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(filters) = &self.filters {
            let mut ctx = Context::new();
            ctx.insert("k", "filters");
            ctx.insert("v", &format!("[{}]", &filters.iter().map(|v| v.bedrock_serialize()).collect::<Vec<String>>().join(",")));

            res.push_str(
                &TEMPLATES.render("generic/kv.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(seq) = &self.sequence {
            let mut ctx = Context::new();
            ctx.insert("k", "sequence");
            ctx.insert("v", &format!("[{}]", seq.iter().map(|v| v.bedrock_serialize()).collect::<Vec<String>>().join(",")));

            res.push_str(
                &TEMPLATES.render("generic/kv.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if let Some(set) = &self.set_property {
            let mut ctx = Context::new();
            ctx.insert("k", "set_property");
            ctx.insert("v", &set.serialize());

            res.push_str(
                &TEMPLATES.render("generic/kv.jsont", &ctx).unwrap()
            );
            res.push(',');
        }
        if res.ends_with(',') { res = res.strip_suffix(',').unwrap().to_string() }

        res.push('}');

        res
    }
}