use std::collections::HashMap;
use eo::sjson::SJsonValue;

#[derive(Clone)]
pub struct EntityComponentGroup {
    name: String,
    components: HashMap<String, SJsonValue>
}

impl EntityComponentGroup {
    pub fn new(
        name: impl Into<String>,
        components: HashMap<String, SJsonValue>
    ) -> Self {
        Self { name: name.into(), components }
    }
}