use std::collections::HashMap;
use eo::sjson::SJsonValue;
use crate::bedrock::BedrockSerializable;

#[derive(Clone)]
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